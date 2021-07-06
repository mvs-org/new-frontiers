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

async function sendTransaction(_privateKey,_from,_to,_value){

    
// var Tx = require('ethereumjs-tx').Transaction;
// const privateKey = Buffer.from(_privateKey, 'hex');


// var rawTx = {
//   nonce: '0x',
//   gasPrice: '0x09184e72a000',
//   gasLimit: '0x2710',
//   to: _to,
//   value: '0x3039',
//   data: '0x7f7465737432000000000000000000000000000000000000000000000000000000600057',
//   chainId: "0"
// }

// var tx = new Tx(rawTx);
// tx.sign(privateKey);

// var serializedTx = tx.serialize();

// web3.eth.sendSignedTransaction('0x' + serializedTx.toString('hex'))
// .on('receipt', console.log);



return new Promise(function(resolve,reject){
    try {
        web3.eth.getBlock("latest", false, (error, result) => {
            console.log('gas limit: '+result.gasLimit)
            var _gasLimit = result.gasLimit;

            web3.eth.getGasPrice(function(error,result){ 
                console.log('gas price: ' + result);
                var _gasPrice = result;


                const Tx = require('ethereumjs-tx').Transaction

                const privateKey = Buffer.from(_privateKey, 'hex')

                var _hex_gasLimit = web3.utils.toHex(_gasLimit.toString());
                var _hex_gasPrice = web3.utils.toHex(_gasPrice.toString());
                var _hex_value = web3.utils.toHex(web3.utils.toWei(_value,'ether'));
                //var _trx_count = web3.eth.getTransactionCount(_from);
                var _hex_Gas = web3.utils.toWei('10', 'Gwei')

                console.log('------------------------------------------');
                web3.eth.getTransactionCount(_from).then(
                    nonce=>{

                        var _hex_nonce = web3.utils.toHex(nonce);
                        const rawTx = {
                            nonce: _hex_nonce,
                            to: _to,
                            from:_from,
                            gasLimit:_hex_gasLimit,
                            gas:_hex_Gas,
                            gasPrice:_hex_gasPrice,
                            value: _hex_value,
                            data: '0x00',
                            chain: '0x'
                        } 

                        const tx = new Tx(rawTx);
                        tx.sign(privateKey);
                        var serializedTx = tx.serialize();

                        web3.eth.sendSignedTransaction('0x' + serializedTx.toString('hex')).on('receipt', console.log);
                        


                })
            });
        });
    } catch (error) {
        resolve(error);
    }
})
}


async function run(){

  accounts[1] = await web3.eth.accounts.create();
  sendTransaction('e339f846630758580c298c18d844e315d36143e2ab036856cbbc296e6391f60e', '0xa6f101a982fdd1ef115a614bdbbf67da71a6c4e3', accounts[1].address, "123456789");
  var balance = await web3.eth.getBalance(accounts[0].address);
  console.log('^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^');
  console.log("main",balance);
  console.log('^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^');
  var balance = await web3.eth.getBalance(accounts[1].address);
  console.log("second", balance);
}

run();