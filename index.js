import init, { gamePrint, gameTick, gameOver, gameScore, moveLeft, moveRight, moveDown, rotate } from './pkg/tetris';

await init();
await main();

async function main() {
  let app = document.getElementById("tetris-app");
  let scoreContainer = document.getElementById("tetris-score");
  app.innerText = gamePrint();

  let playBtn = document.getElementById("tetris-play-btn");
  playBtn.addEventListener("click", async (e) => {
    await init();

    document.onkeydown = (event) => {
      event = event || window.event;
      let key = event.key || event.keyCode;

      switch (key) {
        case 37:
        case "ArrowLeft":
          event.preventDefault();
          moveLeft();
          app.innerText = gamePrint();
          break;

        case 39:
        case "ArrowRight":
          event.preventDefault();
          moveRight();
          app.innerText = gamePrint();
          break;

        case 40:
        case "ArrowDown":
          event.preventDefault();
          moveDown();
          app.innerText = gamePrint();
          break;

        case 32:
        case "Space":
        case " ":
          event.preventDefault();
          rotate();
          app.innerText = gamePrint();
          break;
      }

    };

    let gameSpeed = 300;
    const loopId = setInterval(() => {
      gameTick();
      app.innerText = gamePrint();
      scoreContainer.innerText = gameScore();
      if (gameOver()) {
        scoreContainer.innerText += "   ⛔ Game Over ⛔";
        clearInterval(loopId);
      }
      console.log('the clock is ticking');
    }, gameSpeed);

  });
}