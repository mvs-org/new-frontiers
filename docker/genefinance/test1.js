const fs = require('fs');
const Web3 = require('web3');
// const web3 = new Web3('http://172.30.0.10:9933');
const web3 = new Web3('http://127.0.0.1:9933');
var Accounts = require('web3-eth-accounts');


let accounts = [
  {
    // Develop 1
    address: '0xa6f101a982fdd1ef115a614bdbbf67da71a6c4e3',
    key: '0xe339f846630758580c298c18d844e315d36143e2ab036856cbbc296e6391f60e'
  },
]


async function run() {
    console.log('test******************************************');
    accounts[1] = await web3.eth.accounts.create();
    console.log(accounts);
}

run();