<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const score = ref(0);
const gameOver = ref(false);
const gameStarted = ref(false);
const canvasRef = ref(null);
const gamepadConnected = ref(false);

const CELL_SIZE = 20;
const CANVAS_WIDTH = 600;
const CANVAS_HEIGHT = 400;

const gameState = ref(null);

async function startGame() {
  gameStarted.value = true;
  gameOver.value = false;
  score.value = 0;
  
  // 初始化游戏状态
  gameState.value = await invoke('init_game', {
    width: CANVAS_WIDTH,
    height: CANVAS_HEIGHT,
    cellSize: CELL_SIZE
  });
  
  gameLoop();
}

async function gameLoop() {
  if (!gameStarted.value || gameOver.value) return;

  // 更新游戏状态
  const newState = await invoke('update_game');
  if (newState) {
    gameState.value = newState;
    score.value = newState.score;
    gameOver.value = newState.game_over;
  }

  draw();
  setTimeout(gameLoop, 100);
}

function draw() {
  const ctx = canvasRef.value.getContext('2d');
  ctx.clearRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);

  // 绘制蛇
  ctx.fillStyle = '#4CAF50';
  gameState.value.snake.forEach(segment => {
    ctx.fillRect(segment.x * CELL_SIZE, segment.y * CELL_SIZE, CELL_SIZE - 1, CELL_SIZE - 1);
  });

  // 绘制食物
  ctx.fillStyle = '#FF5722';
  ctx.fillRect(gameState.value.food.x * CELL_SIZE, gameState.value.food.y * CELL_SIZE, CELL_SIZE - 1, CELL_SIZE - 1);
}

async function handleKeydown(e) {
  if (!gameStarted.value) return;

  let direction;
  switch (e.key) {
    case 'ArrowUp':
      direction = 'Up';
      break;
    case 'ArrowDown':
      direction = 'Down';
      break;
    case 'ArrowLeft':
      direction = 'Left';
      break;
    case 'ArrowRight':
      direction = 'Right';
      break;
    default:
      return;
  }

  await invoke('change_direction', { direction });
}

async function handleGamepadInput() {
  if (!gameStarted.value || gameOver.value) return;
  
  const gamepads = navigator.getGamepads();
  const gamepad = gamepads[0]; // 使用第一个连接的手柄
  
  if (!gamepad) return;
  
  // 检测手柄方向键或摇杆输入
  let direction;
  const axes = gamepad.axes;
  const buttons = gamepad.buttons;
  
  // 方向键
  if (buttons[12]?.pressed) { // 上
    direction = 'Up';
  } else if (buttons[13]?.pressed) { // 下
    direction = 'Down';
  } else if (buttons[14]?.pressed) { // 左
    direction = 'Left';
  } else if (buttons[15]?.pressed) { // 右
    direction = 'Right';
  }
  
  // 摇杆
  const STICK_THRESHOLD = 0.5;
  if (Math.abs(axes[0]) > STICK_THRESHOLD || Math.abs(axes[1]) > STICK_THRESHOLD) {
    if (Math.abs(axes[1]) > Math.abs(axes[0])) {
      direction = axes[1] < -STICK_THRESHOLD ? 'Up' : 'Down';
    } else {
      direction = axes[0] < -STICK_THRESHOLD ? 'Left' : 'Right';
    }
  }
  
  if (direction) {
    await invoke('change_direction', { direction });
  }
}

function gamepadLoop() {
  if (gameStarted.value && !gameOver.value) {
    handleGamepadInput();
  }
  requestAnimationFrame(gamepadLoop);
}

function handleGamepadConnected(e) {
  console.log("手柄已连接:", e.gamepad);
  gamepadConnected.value = true;
  gamepadLoop();
}

function handleGamepadDisconnected() {
  console.log("手柄已断开连接");
  gamepadConnected.value = false;
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
  window.addEventListener('gamepadconnected', handleGamepadConnected);
  window.addEventListener('gamepaddisconnected', handleGamepadDisconnected);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
  window.removeEventListener('gamepadconnected', handleGamepadConnected);
  window.removeEventListener('gamepaddisconnected', handleGamepadDisconnected);
});
</script>

<template>
  <div class="container">
    <h1>贪吃蛇游戏</h1>
    
    <div class="game-container">
      <canvas
        ref="canvasRef"
        :width="CANVAS_WIDTH"
        :height="CANVAS_HEIGHT"
        class="game-canvas"
      ></canvas>
      
      <div class="game-info">
        <p class="score">分数: {{ score }}</p>
        <p class="gamepad-status" :class="{ 'connected': gamepadConnected }">
          {{ gamepadConnected ? '游戏手柄已连接' : '未检测到游戏手柄' }}
        </p>
        <button
          @click="startGame"
          class="start-button"
        >
          {{ gameStarted && !gameOver ? '重新开始' : '开始游戏' }}
        </button>
        
        <div v-if="gameOver" class="game-over">
          游戏结束!
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
:global(html), :global(body) {
  margin: 0;
  padding: 0;
  overflow: hidden;
}

.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  height: 100vh;
  padding: 20px;
  box-sizing: border-box;
  overflow: hidden;
}

.game-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
}

.game-canvas {
  border: 2px solid #333;
  background-color: #f0f0f0;
}

.game-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.score {
  font-size: 24px;
  font-weight: bold;
  margin: 0;
}

.start-button {
  padding: 10px 20px;
  font-size: 18px;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.start-button:hover {
  background-color: #45a049;
}

.game-over {
  font-size: 24px;
  color: #ff0000;
  font-weight: bold;
  margin-top: 10px;
}

.gamepad-status {
  font-size: 16px;
  margin: 5px 0;
  color: #666;
}

.gamepad-status.connected {
  color: #4CAF50;
}
</style>
