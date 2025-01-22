//SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

contract MyContract { 
    string value;

    constructor() {
        value = "Hello Harish";
    }

    function get() public view returns(string memory){
        return value;
    }
    function set(string memory _value) public{
        value  = _value;
    }
}