<template>
  <v-container fluid>
    <v-row justify="center">

      <v-col cols="12" sm="6">
        <v-text-field
          label="Formula"
          outlined
          @keydown="formulaChanged"
          v-model="formula"
          @keyup.enter.native="evaluate"
        ></v-text-field>
      </v-col>

      <v-flex xs12 text-xs-center v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error">
          {{ error }}
        </v-alert>
      </v-flex>

      <v-col cols="12">
        <h3>{{ result }}</h3>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { Native, Message } from '@/native';
import { GuiExpression } from '@/native/messages/calculator';

const DIGITS = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
const OPERATORS = ['+', '-', '*', '/'];

@Component
export default class Calculator extends Vue {
  // props
  // data
  formula = '';
  result = '';
  error = '';

  // computed
  // lifecycle
  // watch
  // methods
  formulaChanged(change: KeyboardEvent) {
    const { key } = change;
    if (DIGITS.includes(key)) {
      this.formula += key;
    } else if (OPERATORS.includes(key)) {
      this.formula += ` ${key} `;
    } else if (key === 'Backspace') {
      return;
    }

    change.preventDefault();
  }

  async evaluate() {
    const message: Message = {
      type: 'calculator.gui.expression',
      data: {
        expression: this.formula,
      },
    };
    try {
      const res = await Native.call(message);
      this.result = (res.data as any).result;
    } catch (err) {
      this.error = err.message;
    }
  }
}
</script>


<style lang="scss" scoped>
</style>
