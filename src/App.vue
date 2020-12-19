<template>
  <div class="content">
    <textarea class="left" @input="convert" v-model="inputText"></textarea>
    <div class="right" v-html="convertText"></div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { pulldown_cmark } from "../wasm-markdown/pkg/wasm_markdown";

let wasmContainer: { pulldown_cmark: typeof pulldown_cmark };
import('../wasm-markdown/pkg').then(wasm => (wasmContainer = wasm));

export default defineComponent({
  name: 'App',
  components: {},
  setup() {
    const inputText = ref('');
    const convertText = ref('');
    const convert = () => {
      convertText.value = wasmContainer?.pulldown_cmark(inputText.value);
    };
    return { convert, inputText, convertText };
  }
})

</script>

<style scoped>
.content {
  display: flex;
  width: 100%;
  height: 100vh;
}
.left {
  width: 50%;
}
.right {
  width: 50%;
}
</style>
