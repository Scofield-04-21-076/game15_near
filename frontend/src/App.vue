<template>
  <div class="block-container">
    <div>
      <button v-if="!signedIn" @click="login">
        <span> Connect </span>
      </button>
      <div v-else>
        {{ getAccountId() }}
        <button @click="logout">
          <div class="icon-logout"></div>
        </button>
        <div>
          <button @click="addPlayers">
            <span> Add me to players</span>
          </button>
        </div>
        <div>
          Price (NEAR):
          <input 
            v-bind:value="priceAmount"
            @input="priceAmount = $event.target.value"
            type="number" 
            min="0" 
            max="99999999"
          />
          <button @click="putPrice">Put</button>
        </div>
        <div>
          <button @click="withdrawPrice">Withdraw Price</button>
        </div>
      </div>
    </div>
  </div>

  <div class="block-container">
    <div>
      Players (price):
    </div>
    <div>
      <ul v-for="element in players"
          :key="element">
        <li>
          {{ element["player"] }} ({{ element["price"] }} NEAR)
        </li>
      </ul>
    </div>
  </div>

  <div class="block-container">
    <div>
      You: {{ getAccountId() }} is play: {{ myIsPlay }}
    </div>
    <div>
      Opponent: {{ opponentId }} is play: {{ opponentIsPlay }}
    </div>
  </div>

  <div class="block-container">
    <div>
      Opponent
      <input 
        v-bind:value="opponent_input"
        @input="opponent_input = $event.target.value"
        type="string" 
      />
      <button @click="setOpponentButton(opponent_input)">Set</button>
      <div>
        <button @click="newGameStart">
          <span> Start </span>
        </button>
      </div>
    </div>
  </div>

  <div class="block-container">
    <div class="wrapper">
      <div class="grid">
        <div 
          v-for="tile in state"
          :key="tile"
        >
          <button class="button list" @click="run(tile)">{{ tile }}</button>
        </div>
      </div>
    </div>

    Opponent:
    <div class="wrapper">
      <div class="grid">
        <div 
          v-for="tile in stateOpponent"
          :key="tile"
        >
          <button class="button list">{{ tile }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { getOpponent } from './near/utils';
import { getTiles } from './near/utils';
import { setOpponent } from './near/utils';
import { addMeToPlayers, getPlayers, run, setPrice, isPlayPlayer } from './near/utils';
import { isSignedIn, login, logout, newGame, withdrawCancelPrice } from './near/utils';

const FIFTEEN = Array.from({length: 15}, (e, i) => i + 1);
FIFTEEN.push(0);

const FIFTEEN_KEY = "FIFTEEN";
const YOCTO = 1_000_000_000_000_000_000_000_000;

export default {
  data() {
    return{
      fifteen: FIFTEEN,
      state: [...FIFTEEN],
      stateOpponent: [],
      players: [],
      priceAmount: 0,
      myIsPlay: "",
      opponentId: "",
      opponentIsPlay: "",
      opponent_input: ""
    }
  },  
  methods:{
    login: login,
    logout: logout,

    isSignedIn () {
      return isSignedIn();
    },

    getAccountId() {
      return window.walletConnection.getAccountId();
    },

    async newGameStart() {
      do {
        this.state.sort(() => Math.random() - 0.5);
      } while (!this.isSolvable());

      localStorage.setItem(FIFTEEN_KEY, this.state);

      await newGame(this.state);
    },

    getAndSetState() {
      const localState = localStorage.getItem(FIFTEEN_KEY);
      
      if(localState !== null) {
        this.state = this.stringStateToArray(localStorage.getItem(FIFTEEN_KEY));
      }
    },

    stringStateToArray(stringState) {
      let state = stringState.split(',').map(Number);
      return state;
    },

    isSolvable() {
      let countInversion = 0;
      
      for(let i = 0; i < this.state.length - 1; i++) {
        for(let j = 0; j < i; j++) {
          if (this.state[j] > this.state[i]) {
            countInversion++;
          }
        }
      }

      return countInversion % 2 == 0;
    },

    isPlayable(zeroIndex, tileIndex, width) {
      return (zeroIndex % width !== 0 && zeroIndex - 1 === tileIndex) ||
      (zeroIndex % width !== width - 1 && zeroIndex + 1 === tileIndex) ||
      zeroIndex - width === tileIndex ||
      zeroIndex + width === tileIndex
    },

    async run(tile) {
      const zeroIndex = this.state.indexOf(0);
      const tileIndex = this.state.indexOf(tile);
      if (this.isPlayable(zeroIndex, tileIndex, 4)) {
        this.updateState(tileIndex);
        await run(this.state);
      }
    },

    updateState(tileIndex) {
      const updated = [...this.state];

      updated[this.state.indexOf(0)] = this.state[tileIndex];
      updated[tileIndex] = 0;
      this.state = updated;
      localStorage.setItem(FIFTEEN_KEY, this.state);
    },

    async addPlayers() {
      await addMeToPlayers();
    },

    async putPrice() {
      await setPrice(this.priceAmount);
    },

    async withdrawPrice() {
      await withdrawCancelPrice();
    },

    async setOpponentButton(opponentId) {
      await setOpponent(opponentId);
    },

    isSolved(tiles) {
      if(tiles[tiles.length - 1] != 0) {
        return false;
      }

      for(let i = tiles.length - 1 - 1; i >= 0; i--) {
        if(tiles[i] != i + 1){
          return false;
        }
      }

      return true;
    },

    showPlayers() {
      var players;
      let myIsPlay;
      let opponentIsPlay;
      let opponentId;

      setInterval( () => {
        players = getPlayers()
        players.then(
          (result) => {
            for (let i = 0, k = result.length / 2; i < result.length / 2; i++, k++) {
              for(let j = 0; j < result.length; j++) {
                this.players[j] = {
                  "player": result[i][j],
                  "price": result[k][j].price / YOCTO
                };
              }
            }
          },
          // eslint-disable-next-line
          (reason) => { }
        )
        myIsPlay = isPlayPlayer(this.getAccountId());
        myIsPlay.then(
          (result) => {
            this.myIsPlay = result
          },
          
        )
        opponentIsPlay = isPlayPlayer(this.opponentId);
        opponentIsPlay.then(
          (result) => {
            this.opponentIsPlay = result
          },
          // eslint-disable-next-line
          (reason) => { }
        )
        opponentId = getOpponent(this.getAccountId());
        opponentId.then(
          (result) => {
            this.opponentId = result
          },
          // eslint-disable-next-line
          (reason) => { }
        )
      }, 1000)
    },

    showOpponentTiles() {
      let state;
      setInterval( () => {
        state = getTiles(this.opponentId);
        state.then(
          (result) => {
            this.stateOpponent = result
          },
          // eslint-disable-next-line
          (reason) => { }
        )
      }, 1000)

    },

    checkWin() {
      setInterval( () => {
        if(this.isSolved(this.state) && this.myIsPlay) {
          this.state = [...FIFTEEN];
          alert("You win!!!")
        }
        if(this.isSolved(this.stateOpponent)) {
          alert("You lost")
        }
      }, 1000)
    }
  },
  computed: {
    signedIn() { return window.walletConnection.isSignedIn() }
  },
  mounted () {
    this.getAndSetState();
    this.showPlayers();
    this.showOpponentTiles();
    this.checkWin();
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
.block-container{
  padding: 15px;
  border: 2px solid teal;
  margin: 15px;
}
.icon-logout {
    background-image: url(./assets/Vector.png);
    background-repeat: no-repeat;
    background-position: center;
    width: 20px;
    height: 20px;
    min-width: 20px;
    min-height: 20px;
    display: block;
    background-size: contain;
    background-position: center;
    filter: brightness(5);
}
.button {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  width: 100%;
  padding: 0;
  border-radius: 8px;
  border: 1px solid teal;
  font-size: 18px;
  font-family: inherit;
  background: #fff;
  cursor: pointer;
}
.button:focus {
  outline: none;
  color: #fff;
  background: rgb(5, 65, 65);
}

.button:focus:active {
  background: #fff;
  color: inherit;
}

.button:disabled {
  color: inherit;
  cursor: default;
}

.wrapper {
  position: relative;
  width: 95vmin;
  height: 95vmin;
  max-width: 500px;
  max-height: 500px;
  border-radius: 8px;
  list-style: none;
  overflow: hidden;
  padding: 8px;
}
.grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  grid-gap: 4px;
  height: 100%;
  width: 100%;
  margin: 0;
  padding: 0;
  list-style: none;
}
.item {
  user-select: none;
  cursor: pointer;
}
.list-move {
  transition: transform 0.4s ease;
}
</style>
