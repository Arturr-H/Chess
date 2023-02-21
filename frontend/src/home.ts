// This file helps display the game listing page

/* HTML elements */
const GAMES_CONTAINER = document.getElementById("games-container");

/* Interfaces */
interface GameInfo {
    id: string,
    creator: string,
    minutes: number
}

/* Websocket events - we get the `ws` variable from ./index.ts */
ws.onopen = (e) => {
    /* Send request to get available rooms */
    ws.send(JSON.stringify({
        "request_type": "list_games",
    }));
};

/* Functions */
const display_games = (games: Array<GameInfo>) => {
    console.log(games);
    GAMES_CONTAINER!.innerHTML = "";

    /* Display */
    games.forEach(game => {
        const el = document.createElement("div");
        el.classList.add("game-section");
        el.onclick = () => join_game(game.id);

        const minutes_text = document.createElement("p");
        minutes_text.innerHTML = game.minutes + " minutes";

        const creator_text = document.createElement("p");
        creator_text.innerHTML = "@" + game.creator;
        creator_text.classList.add("highlight");

        el.appendChild(creator_text);
        el.appendChild(minutes_text);

        GAMES_CONTAINER?.appendChild(el);
    });
};

const join_game = (id: string) => {
    ws.send(JSON.stringify({
        "request_type": "join",
        "game_id": id
    }));
};
