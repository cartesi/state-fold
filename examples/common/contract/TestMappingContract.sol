pragma solidity ^0.7.0;

contract TestMappingContract {
    mapping(uint256 => uint256) map;

    event Modified(uint256 key, uint256 value);
    event Removed(uint256 key);

    constructor() {
        modify(1, 20);
        modify(2, 12);
        remove(1);
        modify(3, 9);
        modify(4, 14);
        remove(2);
        modify(5, 97);
    }

    function modify(uint256 key, uint256 value) public {
        map[key] = value;

        emit Modified(key, value);
    }

    function remove(uint256 key) public {
        map[key] = 0;

        emit Removed(key);
    }
}
