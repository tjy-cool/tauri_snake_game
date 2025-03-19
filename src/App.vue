<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from "vue-i18n";

const score = ref(0);
const gameOver = ref(false);
const gameStarted = ref(false);
const canvasRef = ref(null);
const gamepadConnected = ref(false);
const { t, locale } = useI18n();

function toggleLanguage() {
  locale.value = locale.value === 'zh' ? 'en' : 'zh';
}

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
  if (!gameStarted.value) return;

  // 更新游戏状态
  const newState = await invoke('update_game');
  if (newState) {
    gameState.value = newState;
    score.value = newState.score;
    gameOver.value = newState.game_over;
  }

  draw();
  if (!gameOver.value) {
    setTimeout(gameLoop, 100);
  }
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
    <div class="game-card">
      <div class="game-header">
        <div class="score-box">
          <p class="score">{{ t('score', { score }) }}</p>
        </div>
        <h1 class="game-title">{{ t('title') }}</h1>
        <button @click="toggleLanguage" class="language-button">
          {{ locale === 'zh' ? 'English' : '中文' }}
        </button>
      </div>

      <canvas
        ref="canvasRef"
        :width="CANVAS_WIDTH"
        :height="CANVAS_HEIGHT"
        class="game-canvas"
      ></canvas>
      
      <div class="control-panel">
        <button
          @click="startGame"
          class="start-button"
        >
          {{ t(gameStarted && !gameOver ? 'restartGame' : 'startGame') }}
        </button>
        <p class="gamepad-status" :class="{ 'connected': gamepadConnected }">
          {{ t(gamepadConnected ? 'gamepadConnected' : 'gamepadDisconnected') }}
        </p>
      </div>
      
      <div v-if="gameOver" class="game-over">
        {{ t('gameOver') }}
      </div>
    </div>
  </div>
</template>

<style scoped>
:global(html), :global(body) {
  margin: 0;
  padding: 0;
  overflow: hidden;
  background-color: #f0f2f5;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
}

.container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  padding: 20px;
  box-sizing: border-box;
}

.game-card {
  background: white;
  border-radius: 16px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  padding: 24px;
  width: 100%;
  max-width: 800px;
  position: relative;
}

.game-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.game-title {
  font-size: 28px;
  color: #1a1a1a;
  margin: 0;
  font-weight: 600;
  text-align: center;
  flex: 1;
  margin: 0 20px;
}

.score-box {
  border-radius: 8px;
  padding: 8px 16px;
  min-width: 120px;
  padding: 8px 16px;
}

.score {
  font-size: 18px;
  font-weight: 600;
  margin: 0;
  color: #4CAF50;
}

.game-canvas {
  width: 100%;
  background-color: #fafafa;
  border-radius: 12px;
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.05);
  margin-bottom: 24px;
}

.control-panel {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  width: 100%;
  max-width: 600px;
  margin: 0px auto;
  padding: 5px;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  flex-wrap: wrap;
}

.start-button {
  flex: 1;
  min-width: 200px;
  padding: 12px 24px;
  font-size: 16px;
  font-weight: 600;
  background: linear-gradient(135deg, #4CAF50, #45a049);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.start-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(76, 175, 80, 0.3);
}

.gamepad-status {
  flex: 1;
  min-width: 200px;
  text-align: center;
  font-size: 14px;
  margin: 0;
  padding: 8px 16px;
  border-radius: 8px;
  background-color: #f5f5f5;
  color: #666;
  transition: all 0.3s ease;
}

.gamepad-status.connected {
  color: #4CAF50;
  background-color: #E8F5E9;
}

.language-button {
  padding: 8px 16px;
  background: linear-gradient(135deg, #2196F3, #1976D2);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.3s ease;
}

.language-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(33, 150, 243, 0.3);
}

.game-over {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 32px;
  font-weight: bold;
  color: #f44336;
  background-color: rgba(255, 255, 255, 0.98);
  padding: 24px 48px;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
  backdrop-filter: blur(4px);
}
</style>
