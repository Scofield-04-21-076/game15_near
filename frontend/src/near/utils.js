import { connect, Contract, keyStores, utils, WalletConnection } from 'near-api-js'

import { getConfig } from './config'


const nearConfig = getConfig('testnet')
const YOCTO = 1_000_000_000_000_000_000_000_000;

// ---------------- initialization of the contract ---------------- //
export async function initContract() {

    const near = await connect(Object.assign(
        { deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig))
    window.walletConnection = new WalletConnection(near, 'my-app')
    window.accountId = window.walletConnection.getAccountId()
    window.contract = await new Contract(
        window.walletConnection.account(),
        nearConfig.contractName,
        {
            viewMethods: ['get_tiles', 'is_i_in_players', 'get_players', 'get_opponent', 'is_play_player'],
            changeMethods: ['new_game', 'run', 'add_me_to_players', 'set_price', 'withdraw_and_cancel_price', 'set_opponent'],
        }
    )
}

// ---------------- methods for interacting with the wallet ---------------- //
/**
 * Make logout logs the user out of the wallet
 */
export function logout() {

    window.walletConnection.signOut()
    window.location.replace(window.location.origin + window.location.pathname)
}

/**
 * Make login logs the user in of the wallet
 */
export async function login() {

    window.walletConnection.requestSignIn(nearConfig.contractName)
}

/**
 * Make isSignedIn checks whether the user is logged in to the wallet
 *
 * @return true if the user is logged into the wallet and false otherwise - type: bool
 */
 export function isSignedIn() {

    return window.walletConnection.isSignedIn()
}

/**
 * Make getAccountId returns the name of the account currently connected to the wallet
 *
 * @return the name of the account currently connected to the wallet
 */
 export function getAccountId() {

    return window.walletConnection.getAccountId()
}

/**
 * Make getPossiblyAvailableBalance calculates the maximum amount of funds on the user's account that he can send
 *
 * @return the maximum amount of funds on the user's account with an accuracy of two decimal places
 */
export async function getPossiblyAvailableBalance() {

    const diffPossAva = 0.05;

    const account = window.walletConnection.account();
    const responseBalance = await account.getAccountBalance();
    const availableBalance = responseBalance["available"] / YOCTO;
    const possiblyAvailableBalance = availableBalance - diffPossAva;

    return Number(Math.floor(possiblyAvailableBalance * 100) / 100).toFixed(2);
}

/**
 * Make creationTransactionLink generates a link to the deposit just made
 *
 * @returns a reference to the deposit just made
 */
 export function creationTransactionLink () {

    const params = new URLSearchParams(window.location.search);
    const transactionLink =
        `https://explorer.${nearConfig.networkId}.near.org/transactions/${params.get("transactionHashes")}`;

    return transactionLink;
}

/**
 * Make getSmartContractLink generates a reference to a smart contract
 *
 * @returns a reference to a smart contract
 */
export function getSmartContractLink() {

    return `https://explorer.${nearConfig.networkId}.near.org/accounts/${nearConfig.contractName}`;
    
}


// ---------------- methods for interacting with the contract ---------------- //

export async function newGame(shuffle) {

    await window.contract.new_game({
        args: {shuffle: shuffle},
    })
}

export async function run(tiles) {

    await window.contract.run({
        args: {tiles: tiles},
    })
}

export async function getPlayers() {

    return await window.contract.get_players()
}

export async function addMeToPlayers() {

    await window.contract.add_me_to_players()
}

export async function isIInPlayers() {

    return await window.contract.is_i_in_players()
}

export async function setPrice(amount) {

    amount = utils.format.parseNearAmount(amount.toString());

    return await window.contract.set_price({
        args: {},
        amount: amount
    })
}

export async function withdrawCancelPrice() {

    await window.contract.withdraw_and_cancel_price()
}

export async function setOpponent(accountId) {

    await window.contract.set_opponent({
        args: {opponent_id: accountId},
    })
}

export async function isPlayPlayer(playerId) {

    return await window.contract.is_play_player({
        player_id: playerId
    })
}

export async function getOpponent(accountId) {

    return await window.contract.get_opponent({
        account_id: accountId,
    })
}

export async function getTiles(accountId) {

    return await window.contract.get_tiles({
        account_id: accountId,
    })
}
