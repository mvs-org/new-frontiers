const fs = require('fs');
const Web3 = require('web3');
const web3 = new Web3('http://172.30.0.10:9933');

let accounts = [
  {
    // Develop 1
    address: '0xa6f101a982fdd1ef115a614bdbbf67da71a6c4e3',
    key: '0xe339f846630758580c298c18d844e315d36143e2ab036856cbbc296e6391f60e'
  },
]

async function send(transaction, acc) {
    let gas = await transaction.estimateGas({from: acc.address});
    console.log('gas is : ' + gas);
    console.log('parent address is : ' + transaction._parent._address);
    console.log('abi is : ' +  transaction.encodeABI());
    let options = {
        to  : '0xa6f101a982fdd1ef115a614bdbbf67da71a6c4e3',
        data: transaction.encodeABI(),
        gas : gas,
        gasPrice: web3.utils.toWei("10", "gwei"),
    };
    // console.log(options);
    let signedTransaction = await web3.eth.accounts.signTransaction(options, acc.key);
    // console.log(signedTransaction.rawTransaction);
    return await web3.eth.sendSignedTransaction(signedTransaction.rawTransaction);
}

async function deploy(contractName, contractArgs, acc) {
  //  console.log('$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$');
    let abi = fs.readFileSync('./build/' + contractName + ".abi").toString();
    let bin = fs.readFileSync('./build/' + contractName + ".bin").toString();
    let contract = new web3.eth.Contract(JSON.parse(abi));
    // console.log(contract);
    // console.log('$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$');
    
    let handle = send(contract.deploy({data: "0x" + bin, arguments: contractArgs}), acc);
    // console.log('$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$');
    // console.log(contractName, handle.contractAddress);
    return new web3.eth.Contract(JSON.parse(abi), handle.contractAddress);
}

async function run() {
  // console.log(accounts[0]);  
  let myContract = await deploy("HelloWorld", [1000], accounts[0]);
  let storage = await deploy("Storage", [], accounts[0]);
  // await send(myContract.methods.setStorage(storage._address), accounts[0]);
  // let ret2 = await send(myContract.methods.setValue(9527), accounts[0]);
  // console.log('result: ', ret2);
}

run();


