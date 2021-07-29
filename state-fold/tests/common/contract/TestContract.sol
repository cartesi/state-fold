// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.0;
pragma experimental ABIEncoderV2;

contract TestContract {
    struct Struct {
        address sender;
        uint256 value;
    }

    event Unit();

    event Address(address sender);
    event IndexedAddress(address indexed sender);

    event Integer(uint i);
    event IndexedInteger(uint indexed i);

    event Bytes32(bytes32 h);
    event IndexedBytes32(bytes32 indexed h);

    event Complex(address indexed sender, uint indexed i, bytes32 h);

    event FArray(address indexed sender, uint indexed i, uint[2] arr);
    event VArray(address indexed sender, uint indexed i, uint[] arr);

    uint256 x;

    function increment() public {
        x++;
    }

    function emitEvents() public {
        emit Unit();

        emit Address(msg.sender);
        emit IndexedAddress(msg.sender);

        emit Integer(42);
        emit IndexedInteger(43);
        emit IndexedInteger(block.number);

        emit Bytes32("aa");
        emit IndexedBytes32("bb");

        emit Complex(msg.sender, 43, "cc");

        uint256[2] memory fa = [uint256(1), uint256(2)];
        emit FArray(msg.sender, 43, fa);

        uint[] memory a = new uint[](3);
        a[0] = 1; a[1] = 2; a[2] = 3;
        emit VArray(msg.sender, 43, a);
    }

    function getSender() public view returns(address) {
      return msg.sender;
    }

    function getBlock() public view returns(uint) {
      return block.number;
    }

    function getInteger(uint256 i) public pure returns(uint256) {
      return i;
    }

    function getSmallInteger(uint32 i) public pure returns(uint32) {
      return i;
    }

    function getMultiple(uint256 i, uint256 j) public view returns(address, uint256) {
      return (msg.sender, i + j);
    }

    function getFArray(uint256 i, uint256[3] calldata fa) public view returns(address, uint256[3] memory) {
        uint[3] memory v;
        v[0] = i; v[1] = fa[0]; v[2] = fa[1];
        return (msg.sender, v);
    }

    function getVArray(uint256 i, uint256[] calldata va) public view returns(address, uint256[] memory) {
        uint[] memory v = new uint[](3);
        v[0] = i; v[1] = va[0]; v[2] = va[1];
        return (msg.sender, v);
    }

    function getBytes() public pure returns(bytes memory) {
        return "Hello, World!";
    }


    function getStruct(uint256 value) public view returns(uint256, Struct memory) {
        Struct memory s = Struct(msg.sender, value);
        return (value, s);
    }
}
