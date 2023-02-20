
/* HTML elements */
const BOARD: HTMLElement | null = document.getElementById("board");
const GHOST_PIECE: HTMLElement | null = document.getElementById("ghost-piece");
const GAME_CONTAINER: HTMLElement | null = document.getElementById("game-container");
const STATUS_CONTAINER: HTMLElement | null = document.getElementById("status-container");
const TOAST_CONTAINER: HTMLElement | null = document.getElementById("toast-container");
const CLOCK_WHITE: HTMLElement | null = document.getElementById("clock-white");
const CLOCK_BLACK: HTMLElement | null = document.getElementById("clock-black");


/* Constants */
const GRID_SIZE: number = 8;
const ws = new WebSocket("ws://localhost:8081/");

/* Enums */
enum Color {
    Black,
    White
}

/* Types */
type PieceName = "rook" | "bishop" | "knight" | "queen" | "king" | "pawn";

/* Mutable */
let is_white: boolean | null = null;
let peer_addr: string | null = null;
let game_id: string | null = null;
let is_dragging: boolean = false;
let dragging_piece: Piece | null = null;
let dragging_start: [number, number] | null = null;
let total_moves: number = 0; // Used to determine if counter should be decremented
let white_clock_active: boolean = false;
let black_clock_active: boolean = false;
let pieces: Array<Piece | null> = [
    /* Back */
    { name: "rook", color: Color.Black }, { name: "knight", color: Color.Black }, { name: "bishop", color: Color.Black }, 
    { name: "queen", color: Color.Black }, { name: "king", color: Color.Black }, { name: "bishop", color: Color.Black }, 
    { name: "knight", color: Color.Black }, { name: "rook", color: Color.Black }, 

    /* Pawns */
    { name: "pawn", color: Color.Black }, { name: "pawn", color: Color.Black }, { name: "pawn", color: Color.Black }, { name: "pawn", color: Color.Black }, 
    { name: "pawn", color: Color.Black }, { name: "pawn", color: Color.Black }, { name: "pawn", color: Color.Black }, { name: "pawn", color: Color.Black }, 

    /* Empty */
    null, null, null, null, null, null, null, null,
    null, null, null, null, null, null, null, null,
    null, null, null, null, null, null, null, null,
    null, null, null, null, null, null, null, null,

    /* Pawns */
    { name: "pawn", color: Color.White }, { name: "pawn", color: Color.White }, { name: "pawn", color: Color.White }, { name: "pawn", color: Color.White }, 
    { name: "pawn", color: Color.White }, { name: "pawn", color: Color.White }, { name: "pawn", color: Color.White }, { name: "pawn", color: Color.White }, 

    /* Back */
    { name: "rook", color: Color.White }, { name: "knight", color: Color.White }, { name: "bishop", color: Color.White }, 
    { name: "queen", color: Color.White }, { name: "king", color: Color.White }, { name: "bishop", color: Color.White }, 
    { name: "knight", color: Color.White }, { name: "rook", color: Color.White }, 
];

/* Chess src:s */
let chess_pieces_images = {
    light: {
        "rook": "/pieces/white/rook.svg",
        "queen": "/pieces/white/queen.svg",
        "king": "/pieces/white/king.svg",
        "pawn": "/pieces/white/pawn.svg",
        "bishop": "/pieces/white/bishop.svg",
        "knight": "/pieces/white/knight.svg",
    },
    dark: {
        "rook": "/pieces/black/rook.svg",
        "queen": "/pieces/black/queen.svg",
        "king": "/pieces/black/king.svg",
        "pawn": "/pieces/black/pawn.svg",
        "bishop": "/pieces/black/bishop.svg",
        "knight": "/pieces/black/knight.svg",
    }
}

/* Interfaces */
interface Piece {
    name: PieceName,
    color: Color
}

/* Functions */
const draw_grid = (pcs: any) => {
    pieces = pcs;
    BOARD!.innerHTML = "";
    (!is_white)
        ? (
            BOARD!.style.flexDirection = "column-reverse",
            GAME_CONTAINER!.style.flexDirection = "column-reverse"
        )
        : (
            BOARD!.style.flexDirection = "column",
            GAME_CONTAINER!.style.flexDirection = "column"
        )

    for (let y = 0; y < GRID_SIZE; y++) {
        const ROW: HTMLElement = document.createElement("div");
        ROW.classList.add("row");

        /* Iterate over each tile we'll add */
        for (let x = 0; x < GRID_SIZE; x++) {
            let tile: HTMLElement = document.createElement("div");
            tile.classList.add("tile");
            tile.id = `${x}-${y}`;

            let piece = get_piece(pcs, x, y);
            if (piece !== null) {
                let piece_el = document.createElement("img");

                let image_src = get_image_src(piece.name, piece.color);
                if (image_src !== null) {
                    piece_el.src = image_src;
                };

                piece_el.classList.add("piece")
                tile.appendChild(piece_el);
            }

            /* Append */
            ROW.appendChild(tile);
        };
        
        BOARD?.appendChild(ROW)
    }
};

/* Get element from `pieces` array */
const get_piece = (pcs: any, x: number, y: number): Piece | null => {
    return pcs[8*y + x];
}

/* Get image from piece name */
const get_image_src = (name: PieceName, color: Color): string | null => {
    switch (color) {
        case Color.Black:
            return chess_pieces_images.dark[name];
        case Color.White:
            return chess_pieces_images.light[name];
    }
}

/* Event listeners */
BOARD?.addEventListener("mousedown", (e) => {
    let element = e.target as HTMLElement;
    (element.firstChild as HTMLElement).style.display = "none";

    let [x, y] = element.id.split("-").map((e) => parseInt(e));
    dragging_start = [x, y];
    dragging_piece = get_piece(pieces, x, y);
    is_dragging = true;
    GHOST_PIECE!.style.display = "block";
})
BOARD?.addEventListener("mouseup", (e) => {
    draw_grid(pieces);
    if (is_dragging) {
        GHOST_PIECE!.style.display = "none";
        dragging_piece = null;
        is_dragging = false;

        let element = e.target as HTMLElement;
        send_move_piece(dragging_start!, [parseInt(element.id.split("-")[0]), parseInt(element.id.split("-")[1])]);
    }
})
BOARD?.addEventListener("mousemove", (e) => {
    GHOST_PIECE!.style.display = "block";
    GHOST_PIECE!.style.left = `${e.x}px`;
    GHOST_PIECE!.style.top = `${e.y}px`;
    GHOST_PIECE!.style.background = "url(" + get_image_src(dragging_piece?.name!, dragging_piece?.color!)! + ")";
    GHOST_PIECE!.style.backgroundSize = "contain";
})

/* Move piece */
const send_move_piece = (from: [number, number], to: [number, number]) => {
    ws.send(JSON.stringify({
        "request_type": "move",
        "game_id": game_id,
        "from0": from[0].toString(),
        "from1": from[1].toString(),
        "to0": to[0].toString(),
        "to1": to[1].toString(),
    }));
}
const move_piece = (pieces: any) => {
    pieces = [].concat.apply([], pieces).map((e): Piece | null => {
        if (typeof e === "string") {
            return null;
        }else {
            let name = Object.keys(e["Piece"])[0];
            let color = e["Piece"][name]["color"] === "Black" ? Color.Black : Color.White;
            
            return { name: name.toLocaleLowerCase() as PieceName, color: color }
        };
    });
    draw_grid(pieces);
}

/* Websockets */
ws.onmessage = (e) => {
    let data = JSON.parse(e.data);

    /* If no game was found - create game */
    switch (data.type) {
        case "move":
            move_piece(data.board.pieces);

            /* Highlight moved tiles */
            let { from0, from1, to0, to1 } = data;
            document.getElementById(`${from0}-${from1}`)?.classList.add("highlight");
            document.getElementById(`${to0}-${to1}`)?.classList.add("highlight");

            /* Update moves */
            total_moves++;
            
            /* Update clocks */
            update_clock("white", data.time_left_white, data.turn);
            update_clock("black", data.time_left_black, data.turn);
            
            break;
        case "create":
            if (is_white === null) {
                is_white = data.is_white;
            }if (peer_addr === null) {
                peer_addr = data.peer_addr;
            };
            draw_grid(pieces);
            break;
        case "start":
            if (is_white === null) {
                is_white = data.is_white;
            }if (peer_addr === null) {
                peer_addr = data.peer_addr;
            };
            GAME_CONTAINER!.style.display = "inline-flex";
            STATUS_CONTAINER!.style.display = "none";

            draw_grid(pieces);
            game_id = data.game_id;
            break;

        case "error":
            toast(data.message);
            break;

        case "win":
            move_piece(data.board.pieces);

            /* Highlight moved tiles */
            let { from00, from10, to00, to10 } = data;

            document.getElementById(`${from00}-${from10}`)?.classList.add("highlight");
            document.getElementById(`${to00}-${to10}`)?.classList.add("highlight");

            /* Update clocks */
            update_clock("white", data.time_left_white, data.turn);
            update_clock("black", data.time_left_black, data.turn);

            alert(data.lost + " lost");
            break;

        case "game_not_found":
            ws.send(JSON.stringify({
                "request_type": "create",
            }));
            break;
            
        default:
            break;
    }

}
ws.onclose = (e) => {
    alert("Connection closed!");
}
ws.onopen = (e) => {
    ws.send(JSON.stringify({
        "request_type": "join",
    }));
}

/* Show toast */
const toast = (message: any) => {
    let toast_id = "toast-" + Math.random();
    
    const toast = document.createElement("div");
    toast.classList.add("toast");

    const progressBar = document.createElement("div");
    progressBar.classList.add("progress-bar");

    toast.innerText = message;
    toast.appendChild(progressBar);
    TOAST_CONTAINER?.appendChild(toast);

    setTimeout(() => {
        toast.remove();
    }, 3000);
}

let white_clock_interval: any = null;
let black_clock_interval: any = null;
const update_clock = (clock: "black" | "white", ms: number, turn: "White" | "Black") => {
    let seconds = Math.floor(ms / 1000);
    let minutes = Math.floor(seconds / 60);
    seconds = seconds % 60;

    let seconds_str = seconds.toString();
    let minutes_str = minutes.toString();

    if (seconds < 10) { seconds_str = "0" + seconds_str; }
    if (minutes < 10) { minutes_str = "0" + minutes_str; }
    
    if (clock == "black") {
        CLOCK_BLACK!.innerText = minutes_str + ":" + seconds_str;
        if (black_clock_interval !== null) { clearInterval(black_clock_interval); };
    }else {
        CLOCK_WHITE!.innerText = minutes_str + ":" + seconds_str;
        if (white_clock_interval !== null) { clearInterval(white_clock_interval); };
    }

    if (white_clock_active) { CLOCK_WHITE!.classList.add("clock-active"); }
    else { CLOCK_WHITE!.classList.remove("clock-active"); }

    if (black_clock_active) { CLOCK_BLACK!.classList.add("clock-active"); }
    else { CLOCK_BLACK!.classList.remove("clock-active"); }

    if (ms <= 0) {
        if (clock == "black") {
            alert("White won");
        }else {
            alert("Black won");
        }
        return;
    }else if (total_moves >= 2) {
        if (clock == "black" && turn == "Black") {
            white_clock_active = false;
            black_clock_active = true;

            black_clock_interval = setTimeout(() => {
                update_clock(clock, ms - 100, turn);
            }, 100);
        }else if (clock == "white" && turn == "White") {
            white_clock_active = true;
            black_clock_active = false;

            white_clock_interval = setTimeout(() => {
                update_clock(clock, ms - 100, turn);
            }, 100);
        }
    }
}

/* Main */
draw_grid(pieces);
