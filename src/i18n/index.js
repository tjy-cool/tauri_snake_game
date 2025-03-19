import { createI18n } from 'vue-i18n';

const messages = {
  en: {
    title: 'Snake Game',
    score: 'Score: {score}',
    gamepadConnected: 'Gamepad Connected',
    gamepadDisconnected: 'No Gamepad Detected',
    startGame: 'Start Game',
    restartGame: 'Restart Game',
    gameOver: 'Game Over!'
  },
  zh: {
    title: '贪吃蛇游戏',
    score: '分数: {score}',
    gamepadConnected: '游戏手柄已连接',
    gamepadDisconnected: '未检测到游戏手柄',
    startGame: '开始游戏',
    restartGame: '重新开始',
    gameOver: '游戏结束!'
  }
};

const i18n = createI18n({
  legacy: false,
  locale: 'zh',
  fallbackLocale: 'en',
  messages,
});

export default i18n;