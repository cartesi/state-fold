# Examples

There are a few examples in the `examples` directory that can be a good start for the new comers.

* delegate_example
* delegate_client_example
* delegate_server_example

You can choose also choose upon which Solidity data type your example will run on.

* Array
* Struct
* Mapping

## Run delegate example

While running this example, you can select which data type you want to use by utilising the `-m` command line argument.

The following command will run the delegate example using the Mapping data type:

```bash
cd examples
./run_delegate_example.sh -m Mapping
```

The program output would look like this
```log
Data after modify: {1: 20}
Data after modify: {2: 12, 1: 20}
Data after delete {2: 12}
Data after modify: {2: 12, 3: 9}
Data after modify: {2: 12, 3: 9, 4: 14}
Data after delete {3: 9, 4: 14}
Data after modify: {5: 97, 3: 9, 4: 14}
Current block: 9
ContractCtx { mapping: {5: 97, 3: 9, 4: 14} }
Current block: 10
ContractCtx { mapping: {5: 97, 3: 9, 4: 14} }
Current block: 11
ContractCtx { mapping: {5: 97, 3: 9, 4: 14} }
Current block: 12
ContractCtx { mapping: {5: 97, 3: 9, 4: 14} }
```

## Run delegate server and client examples:

While running these examples, you can also select which data type you want to use by utilising the `-m` command line argument.

The following command will run the delegate server example using the Mapping data type:

```bash
cd examples
./run_delegate_server_example.sh -m Mapping
```
Server will start listening for requests
```log
StateFoldServer listening on [::1]:50051
```

Open another terminal to run client
```bash
cd examples
./run_delegate_client_example.sh
```
Client should receive state from the server
```log
RESPONSE=GetStateResponse { json_state: "state: ContractState { ctx: ContractCtx { mapping: {3: 9, 5: 97, 4: 14} } }" }
```

It's worth noting that if no data type is specified, the default is `Array`.
