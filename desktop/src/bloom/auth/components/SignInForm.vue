<template>
  <v-container grid-list-xl text-xs-center>
    <v-layout row wrap text-xs-center>

      <v-flex xs12 sm8 offset-sm2>
        <v-text-field
          type="text"
          label="Username"
          v-model="username"
          :disabled="isLoading"
          @keyup="lowercaseUsername"
        />
      </v-flex>

      <v-flex xs12 sm8 offset-sm2>
        <v-text-field
          label="Password"
          v-model="password"
          :type="showPassword ? 'text' : 'password'"
          :append-icon="showPassword ? 'mdi-eye-off' : 'mdi-eye'"
          @click:append="showPassword = !showPassword"
          :disabled="isLoading"
          @keyup.enter.native="signIn"
        />
      </v-flex>

      <v-flex xs12 text-xs-center v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error">
          {{ error }}
        </v-alert>
      </v-flex>

      <v-flex xs12 text-xs-center>
        <v-btn color="primary" @click="signIn" :loading="isLoading">
          Sign in
        </v-btn>
      </v-flex>

    </v-layout>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { Mutations } from '@/store';
import { Native, Message } from '@/native';

const { log } = require('@bloom42/astro');

@Component
export default class SignInForm extends Vue {
  // props
  // data
  showPassword = false;
  isLoading = false;
  username = '';
  password = '';
  error: string = '';


  // computed
  // lifecycle
  // watch
  // methods
  async signIn() {
    this.error = '';
    this.isLoading = true;
    const message: Message = {
      type: 'auth.gui.sign_in',
      data: {
        username: this.username,
        password: this.password,
      },
    };

    try {
      const res = await Native.call(message);
      log.debug('success');
      log.debug(res);
      this.$store.commit(Mutations.SIGN_IN.toString());
      this.$router.push({ path: '/' });
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  lowercaseUsername() {
    this.username = this.username.toLowerCase();
  }
}
</script>

<style lang="scss" scoped>
</style>
