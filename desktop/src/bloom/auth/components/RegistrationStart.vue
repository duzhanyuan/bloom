<template>
  <v-container grid-list-xl text-xs-center>
    <v-layout row wrap text-xs-center>

      <v-flex xs12 sm8 offset-sm2>
        <v-text-field
          label="Full name"
          type="text"
          v-model="displayName"
          :rules="fulleNameRules"
          :disabled="isLoading"
           hint="This will be public, 'Sylvain kerkour' for example"
        />
      </v-flex>

      <v-flex xs12 sm8 offset-sm2>
        <v-text-field
          label="Email"
          type="email"
          v-model="email"
          :rules="emailRules"
          :disabled="isLoading"
          @keyup="lowercaseEmail"
        />
      </v-flex>

      <v-flex xs12 text-xs-center v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error">
          {{ error }}
        </v-alert>
      </v-flex>

      <v-flex xs12 sm8 offset-sm2 md6 offset-md3 class="text-xs-left disclaimer mb-5">
        By creating an account, I agree to the
        <a @click="openTermsOfService">Terms of Service</a>
        and <a @click="openPrivacyPolicy">Privacy policy</a>.
      </v-flex>

      <v-flex xs12 text-xs-center id="flex-btn">
        <v-btn color="primary"  @click="register" :loading="isLoading">
          Create Account
        </v-btn>
      </v-flex>

    </v-layout>
  </v-container>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { Native, Message } from '@/native';
import { RegistrationStarted } from '@/native/messages/auth';
import { StorePendingAccount } from '../models';
import { Mutations } from '@/store';

const { shell } = (window as any).require('electron');
const config = require('@/config');


@Component
export default class RegistrationStart extends Vue {
  displayName = '';
  fulleNameRules = [
    (v: string) => !!v || 'Name is required',
  ];
  email = '';
  emailRules = [
    (v: string) => !!v || 'Email is required',
    (v: string) => v.indexOf('@') !== -1 || 'Email is not valid',
  ];
  isLoading = false;
  isValid = false;
  error = '';


  async register() {
    this.isLoading = true;
    this.error = '';
    const message: Message = {
      type: 'auth.registration_start',
      data: {
        display_name: this.displayName,
        email: this.email,
      },
    };
    try {
      const res = await Native.call(message);

      const pendingAccount: StorePendingAccount = {
        email: this.email,
        id: (res.data as RegistrationStarted).id,
      };
      this.$store.commit(Mutations.SET_PENDING_ACCOUNT.toString(), pendingAccount);

      this.$router.push({ path: '/auth/registration/verify' });
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  lowercaseEmail() {
    this.email = this.email.toLowerCase();
  }

  openTermsOfService() {
    shell.openExternal(config.TERMS_URL);
  }

  openPrivacyPolicy() {
    shell.openExternal(config.PRIVACY_URL);
  }
}
</script>


<style lang="scss" scoped>
#flex-btn {
  margin-top: -28px;
}
</style>
