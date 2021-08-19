pragma solidity ^0.7.0;

contract TestArrayContract {
    uint256[] data;

    event Modified(uint256 indexed index, uint256 indexed value);
    event Popped();
    event Pushed(uint256 indexed value);

    constructor() {
        push(10);
        push(20);
        push(30);
        push(40);
        push(50);

        pop();

        modify(2, 35);
    }

    function modify(uint256 index, uint256 value) public {
        data[index] = value;

        emit Modified(index, value);
    }

    function pop() public {
        data.pop();

        emit Popped();
    }

    function push(uint256 value) public {
        data.push(value);

        emit Pushed(value);
    }
}
