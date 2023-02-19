
/* HTML elements */
const BOARD: HTMLElement | null = document.getElementById("board");
const GHOST_PIECE: HTMLElement | null = document.getElementById("ghost-piece");

/* Constants */
const hello: String = "hi awd aaawdwadaw lOLl";
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
let game_id: string | null = null;
let is_dragging: boolean = false;
let dragging_piece: Piece | null = null;
let dragging_start: [number, number] | null = null;
let tiles: Array<HTMLElement> = [];
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
        "rook": "https://upload.wikimedia.org/wikipedia/commons/7/72/Chess_rlt45.svg",
        "queen": "https://upload.wikimedia.org/wikipedia/commons/1/15/Chess_qlt45.svg",
        "king": "https://upload.wikimedia.org/wikipedia/commons/4/42/Chess_klt45.svg",
        "pawn": "https://upload.wikimedia.org/wikipedia/commons/4/45/Chess_plt45.svg",
        "bishop": "https://upload.wikimedia.org/wikipedia/commons/4/45/Chess_plt45.svg",
        "knight": "https://upload.wikimedia.org/wikipedia/commons/7/70/Chess_nlt45.svg",
    },
    dark: {
        "rook": "https://upload.wikimedia.org/wikipedia/commons/f/ff/Chess_rdt45.svg",
        "queen": "https://upload.wikimedia.org/wikipedia/commons/4/47/Chess_qdt45.svg",
        "king": "https://upload.wikimedia.org/wikipedia/commons/f/f0/Chess_kdt45.svg",
        "pawn": "https://upload.wikimedia.org/wikipedia/commons/c/c7/Chess_pdt45.svg",
        "bishop": "https://upload.wikimedia.org/wikipedia/commons/9/98/Chess_bdt45.svg",
        "knight": "https://upload.wikimedia.org/wikipedia/commons/e/ef/Chess_ndt45.svg",
    }
}

/* Interfaces */
interface Piece {
    name: PieceName,
    color: Color
}

/* Functions */
const draw_grid = (pcs: any) => {
    tiles = [];
    BOARD!.innerHTML = "";
    (!is_white)
        ? BOARD!.style.flexDirection = "column-reverse"
        : BOARD!.style.flexDirection = "column"

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
            tiles.push(tile);
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

    let [x, y] = element.id.split("-").map((e) => parseInt(e));
    dragging_start = [x, y];
    dragging_piece = get_piece(pieces, x, y);
    is_dragging = true;
    GHOST_PIECE!.style.display = "block";
})
BOARD?.addEventListener("mouseup", (e) => {
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
    console.log(data);

    /* If no game was found - create game */
    if (data.status !== 200) {
        if (data.status === 404 && data.type === "game_not_found") {
            ws.send(JSON.stringify({
                "request_type": "create",
            }));
        };
    }else {
        switch (data.type) {
            case "move":
                move_piece(data.board.pieces);
                break;
            case "create":
                if (is_white === null) {
                    is_white = data.is_white;
                };
                console.log("is", data.is_white);
                draw_grid(pieces);
                alert("Created game!");
                break;
            case "start":
                if (is_white === null) {
                    is_white = data.is_white;
                };
                console.log("is", data.is_white);
                draw_grid(pieces);

                alert("Started game!");
                game_id = data.game_id;
                break;
            default:
                break;
        }
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

/* Main */
draw_grid(pieces);
