
/* HTML elements */
const BOARD: HTMLElement | null = document.getElementById("board");

/* Constants */
const hello: String = "hi awd aaawdwadaw lOLl";
const GRID_SIZE: number = 8;

/* Enums */
enum Color {
    Black,
    White
}

/* Mutable */
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
    name: "rook" | "bishop" | "knight" | "queen" | "king" | "pawn",
    color: Color
}

/* Functions */
const initialize_grid = () => {
    for (let y = 0; y < GRID_SIZE; y++) {
        const ROW: HTMLElement = document.createElement("div");
        ROW.classList.add("row");

        /* Iterate over each tile we'll add */
        for (let x = 0; x < GRID_SIZE; x++) {
            let tile: HTMLElement = document.createElement("div");
            tile.classList.add("tile");

            let piece = get_piece(x, y);
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
const get_piece = (x: number, y: number): Piece | null => {
    return pieces[8*y + x];
}

/* Get image from piece name */
const get_image_src = (name: "rook" | "bishop" | "knight" | "queen" | "king" | "pawn", color: Color): string | null => {
    switch (color) {
        case Color.Black:
            return chess_pieces_images.dark[name];
        case Color.White:
            return chess_pieces_images.light[name];
    }
}

/* Main */
initialize_grid();
