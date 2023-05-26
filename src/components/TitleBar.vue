<script setup lang="ts">
  import {appWindow} from "@tauri-apps/api/window";

  const handleMin = async () => {
    await appWindow.minimize();
  }

  const handleMax = async()=>{
    await appWindow.toggleMaximize();
  }

  const handleClose = async()=>{
    await appWindow.close();
  }

</script>

<template>
  <div class="title-bar">
    <div class="title-bar-drag" data-tauri-drag-region ></div>
    <div class="btn-group">
      <div class="title-bar-button" title="minimize" @click="handleMin">
        <svg aria-hidden="true" width="10" height="10">
          <path d="M 0,5 10,5 10,6 0,6 Z"></path>
        </svg>
      </div>
      <div class="title-bar-button" title="maximize" @click="handleMax">
        <svg aria-hidden="true" width="10" height="10">
          <path d="M 0,0 0,10 10,10 10,0 Z M 1,1 9,1 9,9 1,9 Z"></path>
        </svg>
      </div>
      <div class="title-bar-button btn-close" title="close" style="width: 46px" @click="handleClose">
        <svg aria-hidden="true" width="10" height="10">
          <path d="M 0,0 0,0.7 4.3,5 0,9.3 0,10 0.7,10 5,5.7 9.3,10 10,10 10,9.3 5.7,5 10,0.7 10,0 9.3,0 5,4.3 0.7,0 Z"></path>
        </svg>
      </div>
    </div>
  </div>
</template>

<style scoped>
  .title-bar {
    width: 100%;
    height: 30px;
    position: fixed;
    text-align: center;
    user-select: none;
    display: flex;
  }
  .title-bar-drag{
    height: 100%;
    flex: 1;
    margin-left: 3px;
  }
  .btn-group {
    height: 100%;
    float: right;
  }

  .title-bar-button {
    display: inline-flex;
    width: 40px;
    height: 30px;
    align-items: center;
    justify-content: center;
    transition:background-color 0.20s ease;
  }

  .title-bar-button svg{
    stroke: #a0a0a0;
  }

  .title-bar-button:hover {
    background: #888;
  }

  .title-bar-button:hover svg{
    stroke: #fff;
  }

  .title-bar-button:hover:active {
    background: #666;
  }

  .title-bar-button.btn-close:hover {
    background: #e81123;
  }
  .title-bar-button.btn-close:hover:active {
    background: #bf0f1d;
  }
</style>