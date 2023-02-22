// This file helps display the game listing page

/* HTML elements */
const GAMES_CONTAINER = document.getElementById("games-container");
const SELECTING_GRID = document.getElementById("selecting-grid");
const CONTROLLER_GHOST_PIECE = document.getElementById("controller-ghost-piece");
const GAMES_LISTING_CONTAINER = document.getElementById("games-listing-container");
const INFO_MESSAGE = document.getElementById("info-message");

/* Mutable */
let is_dragging_controller = false;
let controller_image: string = "/pieces/white/knight.svg";
let controlled_piece: HTMLElement | null = null;
let controlled_piece_name: string = "";

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

/* Generate minutes selection */
const knight_places = [
    [[0, 1], "½"],
    [[1, 0], "1"],
    [[3, 0], "2"],
    [[4, 1], "5"],
    [[4, 3], "10"],
    [[3, 4], "15"],
    [[1, 4], "20"],
    [[0, 3], "30"]
];
const generate_minutes_selection_grid = () => {
    SELECTING_GRID!.innerHTML = "";
    controller_image = "/pieces/white/knight.svg";
    controlled_piece_name = "knight";
    INFO_MESSAGE!.innerText = "How many minutes should each player have?";

    for (let i = 0; i < 5; i++) {
        for (let j = 0; j < 5; j++) {
            const tile = document.createElement("div");
            tile.classList.add("inner-tile");
            setTimeout(() => {
                tile.classList.add("fade-in");
            }, 500 + Math.random()*250);

            let t = knight_places.find((e) => e[0][0] == j && e[0][1] == i);
            if (t) {
                tile.id = `minute-tile-${j}-${i}`;

                /* Create text */
                const text = document.createElement("p");
                text.innerText = t[1].toString();

                tile.appendChild(text);
            };
            if (j == 2 && i == 2) {
                tile.classList.add("highlight");
                tile.id = "center";

                let knight = document.createElement("img");
                knight.src = "/pieces/white/knight.svg";
                knight.draggable = false;
                knight.style.userSelect = "none";
                controlled_piece = knight;

                /* Move knight */
                knight?.addEventListener("mousedown", (e) => {
                    is_dragging_controller = true;
                    CONTROLLER_GHOST_PIECE!.style.left = `${e.x}px`;
                    CONTROLLER_GHOST_PIECE!.style.top = `${e.y}px`;
                    CONTROLLER_GHOST_PIECE!.style.display = "block";
                    CONTROLLER_GHOST_PIECE!.style.background = `url(${controller_image})`;
                    knight.style.display = "none";
                })
  
                tile.appendChild(knight);
            };
            SELECTING_GRID!.appendChild(tile);
        };
    };

    knight_places.forEach(e => {
        let tile = document.getElementById(`minute-tile-${e[0][0]}-${e[0][1]}`);
        tile?.classList.add("highlight");
    });
};
const rook_places = [
    [[4, 2], "Create"],
    [[0, 2], "Join"]
];
const generate_create_or_join_grid = () => {
    SELECTING_GRID!.innerHTML = "";
    controller_image = "/pieces/white/rook.svg";
    controlled_piece_name = "rook";

    for (let i = 0; i < 5; i++) {
        for (let j = 0; j < 5; j++) {
            const tile = document.createElement("div");
            tile.classList.add("inner-tile");
            tile.style.opacity = "1";

            let t = rook_places.find((e) => e[0][0] == j && e[0][1] == i);
            if (t) {
                tile.id = `minute-tile-${j}-${i}`;

                /* Create text */
                const text = document.createElement("p");
                text.innerText = t[1].toString();

                tile.appendChild(text);
            }
            if (j == 2 && i == 2) {
                tile.classList.add("highlight");

                tile.id = "center";
                let rook = document.createElement("img");
                rook.src = "/pieces/white/rook.svg";
                rook.draggable = false;
                rook.style.userSelect = "none";
                controlled_piece = rook;

                /* Move rook */
                rook?.addEventListener("mousedown", (e) => {
                    is_dragging_controller = true;
                    CONTROLLER_GHOST_PIECE!.style.left = `${e.x}px`;
                    CONTROLLER_GHOST_PIECE!.style.top = `${e.y}px`;
                    CONTROLLER_GHOST_PIECE!.style.display = "block";
                    CONTROLLER_GHOST_PIECE!.style.background = `url(${controller_image})`;
                    rook.style.display = "none";
                })
  
                tile.appendChild(rook);
            };
            SELECTING_GRID!.appendChild(tile);
        };
    };

    rook_places.forEach(e => {
        let tile = document.getElementById(`minute-tile-${e[0][0]}-${e[0][1]}`);
        tile?.classList.add("highlight");
    });
};

/* Move knight in minutes select */
const move_knight = (to: number[]) => {
    knight_places.forEach(e => {
        console.log(`minute-tile-${e[0][0]}-${e[0][1]}`);
        let tile = document.getElementById(`minute-tile-${e[0][0]}-${e[0][1]}`);
        tile?.classList.add("fade-out");
        tile!.style.opacity = "0";
    });
    let center = document.getElementById(`center`);
    center!.style.animation = "rotate-90 1s ease-in-out infinite";

    const el = document.getElementById(`minute-tile-${to[0]}-${to[1]}`);
    let knight = document.createElement("img");
    knight.src = "/pieces/white/knight.svg";
    knight.draggable = false;
    knight.style.userSelect = "none";

    el!.style.transition = "all 2s ease-in-out";
    el!.style.marginTop = "100%";

    el?.appendChild(knight);

    setTimeout(() => {
        WAITING_FOR_PLAYER!.style.display = "flex";
        WAITING_FOR_PLAYER!.classList.add("fade-in");
        STATUS_CONTAINER!.style.display = "none";
        CREATE_GAME();
    }, 1000);
}
const move_rook = (to: number[]) => {
    rook_places.forEach(e => {
        if (e[0].toString() !== to.toString()) {
            let tile = document.getElementById(`minute-tile-${e[0][0]}-${e[0][1]}`);
            tile!.style.transition = "opacity 0.5s ease-in-out";
            tile!.style.opacity = "0";
        }
    });
    let center = document.getElementById(`center`);
    center!.style.transition = "opacity 1s ease-in-out";
    center!.style.opacity = "0";

    const el = document.getElementById(`minute-tile-${to[0]}-${to[1]}`);
    let rook = document.createElement("img");
    rook.src = "/pieces/white/rook.svg";
    rook.draggable = false;
    rook.style.userSelect = "none";

    el!.style.transition = "all 2s ease-in-out";
    el!.style.marginTop = "100%";
    el!.style.opacity = "0";

    el?.appendChild(rook);

    setTimeout(() => {
        if (to[0] == 0) {
            STATUS_CONTAINER!.style.display = "none";
            GAMES_LISTING_CONTAINER!.style.display = "flex";
            document.getElementById("games-container")!.classList.add("fade-in");
            
            ws.send(JSON.stringify({
                "request_type": "list_games",
            }));
        }else {
            generate_minutes_selection_grid();
        }
    }, 2000);
}

/* Move controlling piece */
document.addEventListener("mouseup", (e) => {
    if (is_dragging_controller) {
        CONTROLLER_GHOST_PIECE!.style.display = "none";
        is_dragging_controller = false;

        let element = e.target as HTMLElement;
        let split = element.id.split("minute-tile-")[1];
        if (split !== undefined && element.id.indexOf("center") == -1) {
            let x = parseInt(split.split("-")[0]);
            let y = parseInt(split.split("-")[1]);
            
            if (controlled_piece_name == "knight") {
                move_knight([x, y]);
            }else if (controlled_piece_name == "rook") {
                move_rook([x, y]);
            }
        }else {
            controlled_piece!.style.display = "block";
        }
    }
})
document.addEventListener("mousemove", (e) => {
    if (is_dragging_controller) {
        CONTROLLER_GHOST_PIECE!.style.left = `${e.x}px`;
        CONTROLLER_GHOST_PIECE!.style.top = `${e.y}px`;
    }
})

/* Join game */
const join_game = (id: string) => {
    GAMES_LISTING_CONTAINER!.style.display = "none";
    ws.send(JSON.stringify({
        "request_type": "join",
        "game_id": id
    }));
};

/* Main */
// generate_minutes_selection_grid();
generate_create_or_join_grid();
