package web3

const (
	mainnetExplorerURL = "https://vm-explorer.mvs.org"
	testnetExplorerURL = "https://vm-explorer.mvs.org"
	testnetURL         = "https://vm.mvs.org/testnet_rpc/"
	mainnetURL         = "https://vm.mvs.org/mainnet_rpc/"
)

var Networks = map[string]Network{
	"testnet": {
		Name:        "testnet",
		URL:         testnetURL,
		Unit:        "ETP",
		ExplorerURL: testnetExplorerURL,
	},
	"hyperspace": {
		Name:        "hyperspace",
		URL:         mainnetURL,
		Unit:        "ETP",
		ExplorerURL: mainnetExplorerURL,
	},
	"localhost": {
		Name: "localhost",
		URL:  "http://localhost:9933",
		Unit: "ETP",
	},
}

type Network struct {
	Name        string
	URL         string
	ExplorerURL string
	Unit        string
}
