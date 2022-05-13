use async_trait::async_trait;
use futures::future::BoxFuture;
use tokio::sync::Semaphore;

#[async_trait]
pub trait PartitionProvider<Event, PartitionData>
where
    Event: Send,
    PartitionData: Send + Sync,
{
    type ProviderErr: std::error::Error + Send;

    async fn fetch_events_with_range(
        &self,
        data: &PartitionData,
        from_block: u64,
        to_block: u64,
    ) -> std::result::Result<Vec<Event>, Self::ProviderErr>;

    fn should_retry_with_partition(&self, err: &Self::ProviderErr) -> bool;
}

pub struct PartitionEvents<'a, Event, PartitionData, Provider>
where
    Event: Send + Sync,
    PartitionData: Send + Sync,
    Provider: PartitionProvider<Event, PartitionData> + Send + Sync,
{
    semaphore: Semaphore,
    provider: &'a Provider,
    partition_data: &'a PartitionData,
    __phantom: std::marker::PhantomData<Event>,
}

impl<'a, E, D, P> PartitionEvents<'a, E, D, P>
where
    E: Send + Sync,
    D: Send + Sync,
    P: PartitionProvider<E, D> + Send + Sync,
{
    pub fn new(concurrent_workers: usize, provider: &'a P, partition_data: &'a D) -> Self {
        let semaphore = Semaphore::new(concurrent_workers);
        PartitionEvents {
            semaphore,
            provider,
            partition_data,
            __phantom: std::marker::PhantomData,
        }
    }

    pub async fn get_events(
        &'a self,
        start_block: u64,
        end_block: u64,
    ) -> std::result::Result<Vec<E>, Vec<P::ProviderErr>> {
        self.get_events_rec(start_block, end_block).await
    }

    fn get_events_rec(
        &'a self,
        start_block: u64,
        end_block: u64,
    ) -> BoxFuture<'a, std::result::Result<Vec<E>, Vec<P::ProviderErr>>> {
        Box::pin(async move {
            let res = {
                // Make number of concurrent fetches bounded.
                let _permit = self.semaphore.acquire().await;
                self.provider
                    .fetch_events_with_range(self.partition_data, start_block, end_block)
                    .await
            };

            match res {
                Ok(events) => Ok(events),
                Err(err) if self.provider.should_retry_with_partition(&err) => {
                    if start_block >= end_block {
                        Err(vec![err])
                    } else {
                        let middle = {
                            let blocks = 1 + end_block - start_block;
                            let half = blocks / 2;
                            start_block + half - 1
                        };

                        let first_fut = self.get_events_rec(start_block, middle);
                        let second_fut = self.get_events_rec(middle + 1, end_block);

                        let (first_res, second_res) = futures::join!(first_fut, second_fut);

                        match (first_res, second_res) {
                            (Ok(mut first), Ok(second)) => {
                                first.extend(second);
                                Ok(first)
                            }

                            (Err(mut first), Err(second)) => {
                                first.extend(second);
                                Err(first)
                            }

                            (Err(err), _) | (_, Err(err)) => Err(err),
                        }
                    }
                }
                Err(err) => Err(vec![err]),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PartitionEvents;
    use super::PartitionProvider;
    use async_trait::async_trait;

    pub struct MockProvider1 {}
    pub struct MockProviderData {}

    #[async_trait]
    impl PartitionProvider<u64, MockProviderData> for MockProvider1 {
        type ProviderErr = std::io::Error;

        async fn fetch_events_with_range(
            &self,
            _: &MockProviderData,
            start_block: u64,
            end_block: u64,
        ) -> Result<Vec<u64>, Self::ProviderErr> {
            async {
                if start_block == end_block {
                    Ok(vec![start_block])
                } else {
                    Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
                }
            }
            .await
        }

        fn should_retry_with_partition(&self, _: &Self::ProviderErr) -> bool {
            true
        }
    }

    #[tokio::test]
    async fn test_partition_simple1() {
        let provider = MockProvider1 {};
        let partition = PartitionEvents::new(1, &provider, &MockProviderData {});

        let ret = partition.get_events(0, 10000).await;
        assert_eq!((0..=10000).collect::<Vec<u64>>(), ret.unwrap());
    }

    #[tokio::test]
    async fn test_partition_simple2() {
        let provider = MockProvider1 {};
        let partition = PartitionEvents::new(16, &provider, &MockProviderData {});

        let ret = partition.get_events(0, 10000).await;
        assert_eq!((0..=10000).collect::<Vec<u64>>(), ret.unwrap());
    }

    pub struct MockProvider2 {}

    #[async_trait]
    impl PartitionProvider<u64, MockProviderData> for MockProvider2 {
        type ProviderErr = std::io::Error;

        async fn fetch_events_with_range(
            &self,
            _: &MockProviderData,
            start_block: u64,
            end_block: u64,
        ) -> Result<Vec<u64>, Self::ProviderErr> {
            async {
                if end_block - start_block <= 4 {
                    // println!("{} {}", start_block, end_block);
                    Ok((start_block..=end_block).collect::<Vec<u64>>())
                } else {
                    Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
                }
            }
            .await
        }

        fn should_retry_with_partition(&self, _: &Self::ProviderErr) -> bool {
            true
        }
    }

    #[tokio::test]
    async fn test_partition_simple3() {
        let provider = MockProvider2 {};
        let partition = PartitionEvents::new(16, &provider, &MockProviderData {});

        let ret = partition.get_events(0, 10000).await;
        assert_eq!((0..=10000).collect::<Vec<u64>>(), ret.unwrap());
    }
}
