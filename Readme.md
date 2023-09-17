## Subscription to the Netflix contract

### Features 
 +  On deploy the subscription contract gets current whitelisted Tokens and Services and registers as a subscriber 
 +  A user can deploy and withdraw whitelisted tokens
 +  A user can subscribe and unsubscribe to multiple services 
 +  The subscription contract automatically sends tokens valued at subscription dollar price equivalent when called upon by the Neflix Contract
 
### Architecture:
<img src="assets/arch1.jpg" alt="Impl1" width="70%"/>
<img src="assets/arch2.jpg" alt="Impl2" width="70%"/>

 ### Safe price evaluation:

 The subscription contract calculates the token price equivalent in dollars based on LP contracts that have been deployed on the devnet. The interaction folder contains details on deploying two such LPs:
 +  ```lp1.devnet.sh``` using tokens  ```AMS-3a6740``` and ```USDC-79d9a4```  
 +  ```lp2.devnet.sh``` using tokens  ```BMS-e00535``` and ```USDC-79d9a4```  

Regarless of the chosen tokens for forming an LP pair the steps are as follows:
+ Clone the mx-eschange-sc repo (```https://github.com/multiversx/mx-exchange-sc```) 
+ Build the ```mx-exchange-sc/dex/router/``` and the ```/mx-exchange-sc/dex/pair``` contracts 
+ Move the script to the ```mx-exchange-sc/dev/interaction``` folder
+ Load the script
+ Deploy the Router Smart Contract with no Pair Smart Contract address parameter (```deployRouterContract```)
+ Deploy the Pair Smart Contract using the Router Smart Contract address and the two LP tokens as parameters (```deployPairContract```)
+ Upgrade the Router Smart Contract with the Pair Smart Contract address parameter (```upgrade```)
+ Create LP pair (```createPair```), issue LP token (```issueLpToken```) and set roles (```setLocalRoles```)
+ Add the initial liquidity (```addInitialLiquidity```). This defines the ratio of the two tokens
+ Add liquidity (```addLiquidity```). This step is needed in order to initiate the price observation. Make sure the added quantities respect the ratio defined in the add initial liquidity step!
+ (Optional) Deploy safe price contract if you haven't done so already (```deploySafePriceContract```). Only one contract is needed to view any LP tokens price ratio.