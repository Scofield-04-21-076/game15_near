<template>
  <div class="block-container">
    <div>
      <button v-if="!signedIn" class="btn-contact connect" @click="login">
        <span id="buttonConnectContent">Connect</span>
      </button>
      <div class="header-btns-wrapper" v-else>
        <div class="btn">{{ getAccountId() }}</div>
        <button class="btn btn-logout " @click="logout">
          <div class="icon-logout"></div>
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import { isSignedIn, login, logout } from './near/utils';

export default {
  data() {
    return{

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
    }
  },
  computed: {
    signedIn() { return window.walletConnection.isSignedIn() }
  },
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
</style>
