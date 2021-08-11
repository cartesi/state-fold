pragma solidity ^0.7.0;

contract TestStructContract {
    struct Person {
        string name;
        uint256 age;
    }

    Person person;

    event ModifiedName(string name);
    event ModifiedAge(uint256 age);

    constructor() {
        modifyName('John Doe');
        modifyAge(52);

        modifyName('Jon Snow');
        modifyAge(34);

        modifyName('Sir Jorah');
        modifyAge(71);
    }

    function modifyName(string memory name) public {
        person.name = name;

        emit ModifiedName(name);
    }

    function modifyAge(uint256 age) public {
        person.age = age;

        emit ModifiedAge(age);
    }
}
