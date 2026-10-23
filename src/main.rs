#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{
    self, Align2, Color32, FontFamily, FontId, Pos2, Rect, Rounding, Sense, Shape, Stroke, Vec2,
};

// ============================================================================
// STRICT 3-COLOR PALETTE DEFINITIONS
// ============================================================================
const DEEP_CHOCOLATE: Color32 = Color32::from_rgb(59, 35, 20); // #3B2314
const SOFT_SAGE: Color32 = Color32::from_rgb(156, 175, 136);   // #9CAF88
const WARM_CREAM: Color32 = Color32::from_rgb(245, 239, 230);  // #F5EFE6

// ============================================================================
// CORE DOMAIN MODEL: PLAYERS, PIECES, & MOVES
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Player {
    Sente, // Black (Bottom, moves up towards row 0)
    Gote,  // White (Top, moves down towards row 8)
}

impl Player {
    #[inline]
    fn opponent(self) -> Self {
        match self {
            Player::Sente => Player::Gote,
            Player::Gote => Player::Sente,
        }
    }

    #[inline]
    fn forward_dir(self) -> isize {
        match self {
            Player::Sente => -1,
            Player::Gote => 1,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Player::Sente => "Sente (▲ Bottom)",
            Player::Gote => "Gote (▽ Top)",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum PieceKind {
    King,
    Rook,
    Bishop,
    Gold,
    Silver,
    Knight,
    Lance,
    Pawn,
    PromotedRook,   // Dragon King (+R)
    PromotedBishop, // Dragon Horse (+B)
    PromotedSilver, // (+S)
    PromotedKnight, // (+N)
    PromotedLance,  // (+L)
    PromotedPawn,   // Tokin (+P)
}

const HAND_KINDS: [PieceKind; 7] = [
    PieceKind::Rook,
    PieceKind::Bishop,
    PieceKind::Gold,
    PieceKind::Silver,
    PieceKind::Knight,
    PieceKind::Lance,
    PieceKind::Pawn,
];

impl PieceKind {
    fn promote(self) -> Option<Self> {
        match self {
            PieceKind::Rook => Some(PieceKind::PromotedRook),
            PieceKind::Bishop => Some(PieceKind::PromotedBishop),
            PieceKind::Silver => Some(PieceKind::PromotedSilver),
            PieceKind::Knight => Some(PieceKind::PromotedKnight),
            PieceKind::Lance => Some(PieceKind::PromotedLance),
            PieceKind::Pawn => Some(PieceKind::PromotedPawn),
            _ => None,
        }
    }

    fn unpromote(self) -> Self {
        match self {
            PieceKind::PromotedRook => PieceKind::Rook,
            PieceKind::PromotedBishop => PieceKind::Bishop,
            PieceKind::PromotedSilver => PieceKind::Silver,
            PieceKind::PromotedKnight => PieceKind::Knight,
            PieceKind::PromotedLance => PieceKind::Lance,
            PieceKind::PromotedPawn => PieceKind::Pawn,
            other => other,
        }
    }

    fn is_promoted(self) -> bool {
        matches!(
            self,
            PieceKind::PromotedRook
                | PieceKind::PromotedBishop
                | PieceKind::PromotedSilver
                | PieceKind::PromotedKnight
                | PieceKind::PromotedLance
                | PieceKind::PromotedPawn
        )
    }

    fn label(self) -> &'static str {
        match self {
            PieceKind::King => "K",
            PieceKind::Rook => "R",
            PieceKind::Bishop => "B",
            PieceKind::Gold => "G",
            PieceKind::Silver => "S",
            PieceKind::Knight => "N",
            PieceKind::Lance => "L",
            PieceKind::Pawn => "P",
            PieceKind::PromotedRook => "+R",
            PieceKind::PromotedBishop => "+B",
            PieceKind::PromotedSilver => "+S",
            PieceKind::PromotedKnight => "+N",
            PieceKind::PromotedLance => "+L",
            PieceKind::PromotedPawn => "+P",
        }
    }

    fn title(self) -> &'static str {
        match self {
            PieceKind::King => "King (Gyoku)",
            PieceKind::Rook => "Rook (Hisha)",
            PieceKind::Bishop => "Bishop (Kaku)",
            PieceKind::Gold => "Gold (Kin)",
            PieceKind::Silver => "Silver (Gin)",
            PieceKind::Knight => "Knight (Keima)",
            PieceKind::Lance => "Lance (Kyosha)",
            PieceKind::Pawn => "Pawn (Fuhyo)",
            PieceKind::PromotedRook => "Dragon (+R)",
            PieceKind::PromotedBishop => "Horse (+B)",
            PieceKind::PromotedSilver => "Prom. Silver",
            PieceKind::PromotedKnight => "Prom. Knight",
            PieceKind::PromotedLance => "Prom. Lance",
            PieceKind::PromotedPawn => "Tokin (+P)",
        }
    }

    fn material_value(self) -> i32 {
        match self {
            PieceKind::King => 15_000,
            PieceKind::Rook => 950,
            PieceKind::Bishop => 850,
            PieceKind::Gold => 600,
            PieceKind::Silver => 500,
            PieceKind::Knight => 400,
            PieceKind::Lance => 350,
            PieceKind::Pawn => 100,
            PieceKind::PromotedRook => 1_300,
            PieceKind::PromotedBishop => 1_150,
            PieceKind::PromotedSilver => 600,
            PieceKind::PromotedKnight => 600,
            PieceKind::PromotedLance => 600,
            PieceKind::PromotedPawn => 600,
        }
    }

    fn hand_index(self) -> Option<usize> {
        match self.unpromote() {
            PieceKind::Rook => Some(0),
            PieceKind::Bishop => Some(1),
            PieceKind::Gold => Some(2),
            PieceKind::Silver => Some(3),
            PieceKind::Knight => Some(4),
            PieceKind::Lance => Some(5),
            PieceKind::Pawn => Some(6),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Piece {
    kind: PieceKind,
    owner: Player,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    Board {
        from: (usize, usize),
        to: (usize, usize),
        promote: bool,
    },
    Drop {
        kind: PieceKind,
        to: (usize, usize),
    },
}

impl Move {
    fn to_square(self) -> (usize, usize) {
        match self {
            Move::Board { to, .. } | Move::Drop { to, .. } => to,
        }
    }

    fn notation(self, board: &GameState) -> String {
        let file_char = |c: usize| (b'9' - c as u8) as char;
        let rank_char = |r: usize| (b'a' + r as u8) as char;
        match self {
            Move::Board { from, to, promote } => {
                let piece_str = board.grid[from.0][from.1]
                    .map(|p| p.kind.label())
                    .unwrap_or("?");
                let sep = if board.grid[to.0][to.1].is_some() {
                    "x"
                } else {
                    "-"
                };
                let promo = if promote { "+" } else { "" };
                format!(
                    "{}{}{}{}{}{}{}",
                    piece_str,
                    file_char(from.1),
                    rank_char(from.0),
                    sep,
                    file_char(to.1),
                    rank_char(to.0),
                    promo
                )
            }
            Move::Drop { kind, to } => {
                format!("{}*{}{}", kind.label(), file_char(to.1), rank_char(to.0))
            }
        }
    }
}

// ============================================================================
// SHOGI ENGINE & COMPLETE RULESET
// ============================================================================

#[derive(Clone)]
struct GameState {
    grid: [[Option<Piece>; 9]; 9],
    hands: [[u8; 7]; 2], // [0] = Sente, [1] = Gote
    turn: Player,
    last_move: Option<Move>,
}

impl GameState {
    fn new() -> Self {
        let mut grid = [[None; 9]; 9];

        let back_rank = [
            PieceKind::Lance,
            PieceKind::Knight,
            PieceKind::Silver,
            PieceKind::Gold,
            PieceKind::King,
            PieceKind::Gold,
            PieceKind::Silver,
            PieceKind::Knight,
            PieceKind::Lance,
        ];

        for (col, &kind) in back_rank.iter().enumerate() {
            grid[0][col] = Some(Piece {
                kind,
                owner: Player::Gote,
            });
            grid[8][col] = Some(Piece {
                kind,
                owner: Player::Sente,
            });
        }

        // Gote Rook & Bishop
        grid[1][1] = Some(Piece {
            kind: PieceKind::Rook,
            owner: Player::Gote,
        });
        grid[1][7] = Some(Piece {
            kind: PieceKind::Bishop,
            owner: Player::Gote,
        });

        // Sente Bishop & Rook
        grid[7][1] = Some(Piece {
            kind: PieceKind::Bishop,
            owner: Player::Sente,
        });
        grid[7][7] = Some(Piece {
            kind: PieceKind::Rook,
            owner: Player::Sente,
        });

        // Pawns
        for col in 0..9 {
            grid[2][col] = Some(Piece {
                kind: PieceKind::Pawn,
                owner: Player::Gote,
            });
            grid[6][col] = Some(Piece {
                kind: PieceKind::Pawn,
                owner: Player::Sente,
            });
        }

        Self {
            grid,
            hands: [[0; 7]; 2],
            turn: Player::Sente,
            last_move: None,
        }
    }

    #[inline]
    fn hand_ref(&self, player: Player) -> &[u8; 7] {
        match player {
            Player::Sente => &self.hands[0],
            Player::Gote => &self.hands[1],
        }
    }

    #[inline]
    fn hand_mut(&mut self, player: Player) -> &mut [u8; 7] {
        match player {
            Player::Sente => &mut self.hands[0],
            Player::Gote => &mut self.hands[1],
        }
    }

    #[inline]
    fn in_bounds(r: isize, c: isize) -> bool {
        (0..9).contains(&r) && (0..9).contains(&c)
    }

    #[inline]
    fn in_promotion_zone(player: Player, row: usize) -> bool {
        match player {
            Player::Sente => row <= 2,
            Player::Gote => row >= 6,
        }
    }

    #[inline]
    fn must_promote(player: Player, kind: PieceKind, to_row: usize) -> bool {
        match (player, kind) {
            (Player::Sente, PieceKind::Pawn | PieceKind::Lance) => to_row == 0,
            (Player::Gote, PieceKind::Pawn | PieceKind::Lance) => to_row == 8,
            (Player::Sente, PieceKind::Knight) => to_row <= 1,
            (Player::Gote, PieceKind::Knight) => to_row >= 7,
            _ => false,
        }
    }

    fn apply_move_unchecked(&self, mv: Move) -> Self {
        let mut next = self.clone();
        match mv {
            Move::Board { from, to, promote } => {
                if let Some(captured) = next.grid[to.0][to.1].take() {
                    if let Some(idx) = captured.kind.hand_index() {
                        next.hand_mut(self.turn)[idx] += 1;
                    }
                }
                if let Some(mut moving) = next.grid[from.0][from.1].take() {
                    if promote {
                        if let Some(promoted_kind) = moving.kind.promote() {
                            moving.kind = promoted_kind;
                        }
                    }
                    next.grid[to.0][to.1] = Some(moving);
                }
            }
            Move::Drop { kind, to } => {
                if let Some(idx) = kind.hand_index() {
                    let count = &mut next.hand_mut(self.turn)[idx];
                    if *count > 0 {
                        *count -= 1;
                    }
                }
                next.grid[to.0][to.1] = Some(Piece {
                    kind,
                    owner: self.turn,
                });
            }
        }
        next.turn = self.turn.opponent();
        next.last_move = Some(mv);
        next
    }

    fn find_king(&self, player: Player) -> Option<(usize, usize)> {
        for r in 0..9 {
            for c in 0..9 {
                if let Some(p) = self.grid[r][c] {
                    if p.owner == player && p.kind == PieceKind::King {
                        return Some((r, c));
                    }
                }
            }
        }
        None
    }

    fn piece_attacks_square(
        &self,
        from: (usize, usize),
        piece: Piece,
        target: (usize, usize),
    ) -> bool {
        let mut targets = Vec::with_capacity(18);
        self.collect_piece_targets(from, piece, &mut targets);
        targets.contains(&target)
    }

    fn is_in_check(&self, player: Player) -> bool {
        let Some(king_pos) = self.find_king(player) else {
            return true;
        };
        let attacker = player.opponent();
        for r in 0..9 {
            for c in 0..9 {
                if let Some(p) = self.grid[r][c] {
                    if p.owner == attacker && self.piece_attacks_square((r, c), p, king_pos) {
                        return true;
                    }
                }
            }
        }
        false
    }

    #[inline]
    fn push_step(
        &self,
        r: isize,
        c: isize,
        dr: isize,
        dc: isize,
        owner: Player,
        out: &mut Vec<(usize, usize)>,
    ) {
        let nr = r + dr;
        let nc = c + dc;
        if Self::in_bounds(nr, nc) {
            let (ur, uc) = (nr as usize, nc as usize);
            if self.grid[ur][uc].map_or(true, |p| p.owner != owner) {
                out.push((ur, uc));
            }
        }
    }

    #[inline]
    fn push_slide(
        &self,
        r: isize,
        c: isize,
        dr: isize,
        dc: isize,
        owner: Player,
        out: &mut Vec<(usize, usize)>,
    ) {
        let mut nr = r + dr;
        let mut nc = c + dc;
        while Self::in_bounds(nr, nc) {
            let (ur, uc) = (nr as usize, nc as usize);
            match self.grid[ur][uc] {
                None => out.push((ur, uc)),
                Some(occupant) => {
                    if occupant.owner != owner {
                        out.push((ur, uc));
                    }
                    break;
                }
            }
            nr += dr;
            nc += dc;
        }
    }

    fn collect_piece_targets(
        &self,
        from: (usize, usize),
        piece: Piece,
        out: &mut Vec<(usize, usize)>,
    ) {
        let (r, c) = (from.0 as isize, from.1 as isize);
        let owner = piece.owner;
        let fwd = owner.forward_dir();

        match piece.kind {
            PieceKind::King => {
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr != 0 || dc != 0 {
                            self.push_step(r, c, dr, dc, owner, out);
                        }
                    }
                }
            }
            PieceKind::Gold
            | PieceKind::PromotedSilver
            | PieceKind::PromotedKnight
            | PieceKind::PromotedLance
            | PieceKind::PromotedPawn => {
                self.push_step(r, c, fwd, 0, owner, out);
                self.push_step(r, c, fwd, -1, owner, out);
                self.push_step(r, c, fwd, 1, owner, out);
                self.push_step(r, c, 0, -1, owner, out);
                self.push_step(r, c, 0, 1, owner, out);
                self.push_step(r, c, -fwd, 0, owner, out);
            }
            PieceKind::Silver => {
                self.push_step(r, c, fwd, 0, owner, out);
                self.push_step(r, c, fwd, -1, owner, out);
                self.push_step(r, c, fwd, 1, owner, out);
                self.push_step(r, c, -fwd, -1, owner, out);
                self.push_step(r, c, -fwd, 1, owner, out);
            }
            PieceKind::Knight => {
                self.push_step(r, c, fwd * 2, -1, owner, out);
                self.push_step(r, c, fwd * 2, 1, owner, out);
            }
            PieceKind::Pawn => {
                self.push_step(r, c, fwd, 0, owner, out);
            }
            PieceKind::Lance => {
                self.push_slide(r, c, fwd, 0, owner, out);
            }
            PieceKind::Rook => {
                self.push_slide(r, c, 1, 0, owner, out);
                self.push_slide(r, c, -1, 0, owner, out);
                self.push_slide(r, c, 0, 1, owner, out);
                self.push_slide(r, c, 0, -1, owner, out);
            }
            PieceKind::Bishop => {
                self.push_slide(r, c, 1, 1, owner, out);
                self.push_slide(r, c, 1, -1, owner, out);
                self.push_slide(r, c, -1, 1, owner, out);
                self.push_slide(r, c, -1, -1, owner, out);
            }
            PieceKind::PromotedRook => {
                self.push_slide(r, c, 1, 0, owner, out);
                self.push_slide(r, c, -1, 0, owner, out);
                self.push_slide(r, c, 0, 1, owner, out);
                self.push_slide(r, c, 0, -1, owner, out);
                self.push_step(r, c, 1, 1, owner, out);
                self.push_step(r, c, 1, -1, owner, out);
                self.push_step(r, c, -1, 1, owner, out);
                self.push_step(r, c, -1, -1, owner, out);
            }
            PieceKind::PromotedBishop => {
                self.push_slide(r, c, 1, 1, owner, out);
                self.push_slide(r, c, 1, -1, owner, out);
                self.push_slide(r, c, -1, 1, owner, out);
                self.push_slide(r, c, -1, -1, owner, out);
                self.push_step(r, c, 1, 0, owner, out);
                self.push_step(r, c, -1, 0, owner, out);
                self.push_step(r, c, 0, 1, owner, out);
                self.push_step(r, c, 0, -1, owner, out);
            }
        }
    }

    fn generate_pseudo_moves(&self, player: Player) -> Vec<Move> {
        let mut moves = Vec::with_capacity(80);
        let mut targets = Vec::with_capacity(18);

        // 1. Board Moves
        for r in 0..9 {
            for c in 0..9 {
                if let Some(piece) = self.grid[r][c] {
                    if piece.owner != player {
                        continue;
                    }
                    targets.clear();
                    self.collect_piece_targets((r, c), piece, &mut targets);
                    let can_promote_kind = piece.kind.promote().is_some();

                    for &(tr, tc) in &targets {
                        let in_zone = Self::in_promotion_zone(player, r)
                            || Self::in_promotion_zone(player, tr);
                        if can_promote_kind && in_zone {
                            if Self::must_promote(player, piece.kind, tr) {
                                moves.push(Move::Board {
                                    from: (r, c),
                                    to: (tr, tc),
                                    promote: true,
                                });
                            } else {
                                moves.push(Move::Board {
                                    from: (r, c),
                                    to: (tr, tc),
                                    promote: true,
                                });
                                moves.push(Move::Board {
                                    from: (r, c),
                                    to: (tr, tc),
                                    promote: false,
                                });
                            }
                        } else {
                            moves.push(Move::Board {
                                from: (r, c),
                                to: (tr, tc),
                                promote: false,
                            });
                        }
                    }
                }
            }
        }

        // 2. Drop Moves from Hand
        let hand = self.hand_ref(player);
        for (idx, &count) in hand.iter().enumerate() {
            if count == 0 {
                continue;
            }
            let kind = HAND_KINDS[idx];

            for c in 0..9 {
                // Nifu Check: cannot drop unpromoted Pawn on a column containing friendly unpromoted Pawn
                if kind == PieceKind::Pawn {
                    let has_friendly_pawn = (0..9).any(|r| {
                        self.grid[r][c]
                            == Some(Piece {
                            kind: PieceKind::Pawn,
                            owner: player,
                        })
                    });
                    if has_friendly_pawn {
                        continue;
                    }
                }

                for r in 0..9 {
                    if self.grid[r][c].is_some() {
                        continue;
                    }
                    // Dead-end drop restriction
                    if Self::must_promote(player, kind, r) {
                        continue;
                    }
                    moves.push(Move::Drop { kind, to: (r, c) });
                }
            }
        }

        moves
    }

    /// Generates all legal moves (excluding Uchifuzume check, used internally to avoid recursion)
    fn generate_king_safe_moves(&self, player: Player) -> Vec<Move> {
        let pseudo = self.generate_pseudo_moves(player);
        let mut safe = Vec::with_capacity(pseudo.len());
        for mv in pseudo {
            let next = self.apply_move_unchecked(mv);
            if !next.is_in_check(player) {
                safe.push(mv);
            }
        }
        safe
    }

    /// Generates strictly legal Shogi moves including Uchifuzume (Drop-Pawn Mate prohibition)
    fn generate_legal_moves(&self) -> Vec<Move> {
        let player = self.turn;
        let opponent = player.opponent();
        let candidates = self.generate_king_safe_moves(player);
        let mut legal = Vec::with_capacity(candidates.len());

        for mv in candidates {
            if let Move::Drop {
                kind: PieceKind::Pawn,
                ..
            } = mv
            {
                let next = self.apply_move_unchecked(mv);
                if next.is_in_check(opponent) && next.generate_king_safe_moves(opponent).is_empty()
                {
                    // Uchifuzume: illegal to checkmate immediately with a dropped pawn
                    continue;
                }
            }
            legal.push(mv);
        }
        legal
    }

    /// Static evaluation from `Sente`'s perspective for Alpha-Beta AI
    fn evaluate(&self) -> i32 {
        let mut score = 0;

        for r in 0..9 {
            for c in 0..9 {
                if let Some(p) = self.grid[r][c] {
                    let mut val = p.kind.material_value();
                    let advancement = match p.owner {
                        Player::Sente => (8 - r) as i32,
                        Player::Gote => r as i32,
                    };
                    let center_bonus = 4 - (c as i32 - 4).abs();
                    if p.kind != PieceKind::King {
                        val += advancement * 6 + center_bonus * 4;
                    }
                    if p.owner == Player::Sente {
                        score += val;
                    } else {
                        score -= val;
                    }
                }
            }
        }

        for (idx, &kind) in HAND_KINDS.iter().enumerate() {
            let hand_val = (kind.material_value() * 108) / 100;
            score += (self.hands[0][idx] as i32) * hand_val;
            score -= (self.hands[1][idx] as i32) * hand_val;
        }

        score
    }

    fn choose_ai_move(&self, depth: u8) -> Option<Move> {
        let legal = self.generate_legal_moves();
        if legal.is_empty() {
            return None;
        }

        let maximizing = self.turn == Player::Sente;
        let mut best_move = legal[0];
        let mut best_score = if maximizing { i32::MIN + 1 } else { i32::MAX - 1 };
        let mut alpha = i32::MIN + 1;
        let mut beta = i32::MAX - 1;

        for mv in legal {
            let next = self.apply_move_unchecked(mv);
            let eval = Self::alpha_beta(&next, depth.saturating_sub(1), alpha, beta);
            if maximizing {
                if eval > best_score {
                    best_score = eval;
                    best_move = mv;
                }
                alpha = alpha.max(eval);
            } else {
                if eval < best_score {
                    best_score = eval;
                    best_move = mv;
                }
                beta = beta.min(eval);
            }
        }

        Some(best_move)
    }

    fn alpha_beta(state: &GameState, depth: u8, mut alpha: i32, mut beta: i32) -> i32 {
        if depth == 0 {
            return state.evaluate();
        }
        let mut moves = state.generate_king_safe_moves(state.turn);
        if moves.is_empty() {
            return if state.turn == Player::Sente {
                -20_000 - (depth as i32) * 100
            } else {
                20_000 + (depth as i32) * 100
            };
        }

        moves.sort_by_key(|mv| match *mv {
            Move::Board { to, promote, .. } => {
                let cap = state.grid[to.0][to.1].map_or(0, |p| p.kind.material_value());
                -(cap + if promote { 200 } else { 0 })
            }
            Move::Drop { .. } => 0,
        });

        if state.turn == Player::Sente {
            let mut value = i32::MIN + 1;
            for mv in moves {
                let next = state.apply_move_unchecked(mv);
                value = value.max(Self::alpha_beta(&next, depth - 1, alpha, beta));
                alpha = alpha.max(value);
                if alpha >= beta {
                    break;
                }
            }
            value
        } else {
            let mut value = i32::MAX - 1;
            for mv in moves {
                let next = state.apply_move_unchecked(mv);
                value = value.min(Self::alpha_beta(&next, depth - 1, alpha, beta));
                beta = beta.min(value);
                if beta <= alpha {
                    break;
                }
            }
            value
        }
    }
}

// ============================================================================
// UI STATE & STRICT 3-COLOR RENDERING ENGINE
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
enum Selection {
    None,
    BoardSquare(usize, usize),
    HandPiece(PieceKind),
}

struct PendingPromotion {
    from: (usize, usize),
    to: (usize, usize),
}

struct ShogiApp {
    state: GameState,
    history: Vec<GameState>,
    move_log: Vec<String>,
    selection: Selection,
    pending_promotion: Option<PendingPromotion>,
    vs_ai: bool,
    ai_player: Player,
    status_banner: String,
    game_over: bool,
}

impl Default for ShogiApp {
    fn default() -> Self {
        let mut app = Self {
            state: GameState::new(),
            history: Vec::new(),
            move_log: Vec::new(),
            selection: Selection::None,
            pending_promotion: None,
            vs_ai: true,
            ai_player: Player::Gote,
            status_banner: String::new(),
            game_over: false,
        };
        app.refresh_status();
        app
    }
}

impl ShogiApp {
    fn reset_game(&mut self) {
        self.state = GameState::new();
        self.history.clear();
        self.move_log.clear();
        self.selection = Selection::None;
        self.pending_promotion = None;
        self.game_over = false;
        self.refresh_status();
    }

    fn undo_move(&mut self) {
        self.pending_promotion = None;
        self.selection = Selection::None;
        let pop_count = if self.vs_ai && self.history.len() >= 2 {
            2
        } else {
            1
        };
        for _ in 0..pop_count {
            if let Some(prev) = self.history.pop() {
                self.state = prev;
                self.move_log.pop();
            }
        }
        self.game_over = false;
        self.refresh_status();
    }

    fn refresh_status(&mut self) {
        let legal = self.state.generate_legal_moves();
        let in_check = self.state.is_in_check(self.state.turn);
        if legal.is_empty() {
            self.game_over = true;
            let winner = self.state.turn.opponent();
            self.status_banner = format!("CHECKMATE! {} wins the match.", winner.name());
        } else if in_check {
            self.status_banner = format!("CHECK! {} must defend the King.", self.state.turn.name());
        } else {
            self.status_banner = format!("Turn: {}", self.state.turn.name());
        }
    }

    fn commit_move(&mut self, mv: Move) {
        let label = format!(
            "{}. {}",
            self.move_log.len() + 1,
            mv.notation(&self.state)
        );
        self.history.push(self.state.clone());
        self.move_log.push(label);
        self.state = self.state.apply_move_unchecked(mv);
        self.selection = Selection::None;
        self.pending_promotion = None;
        self.refresh_status();
    }

    fn handle_square_click(&mut self, r: usize, c: usize, legal_moves: &[Move]) {
        if self.game_over || self.pending_promotion.is_some() {
            return;
        }
        if self.vs_ai && self.state.turn == self.ai_player {
            return;
        }

        match self.selection {
            Selection::BoardSquare(sr, sc) => {
                if (sr, sc) == (r, c) {
                    self.selection = Selection::None;
                    return;
                }
                let matching: Vec<Move> = legal_moves
                    .iter()
                    .copied()
                    .filter(|mv| matches!(mv, Move::Board { from, to, .. } if *from == (sr, sc) && *to == (r, c)))
                    .collect();

                if matching.len() == 2 {
                    self.pending_promotion = Some(PendingPromotion {
                        from: (sr, sc),
                        to: (r, c),
                    });
                    return;
                } else if let Some(&single_mv) = matching.first() {
                    self.commit_move(single_mv);
                    return;
                }
            }
            Selection::HandPiece(kind) => {
                if let Some(&drop_mv) = legal_moves.iter().find(|mv| {
                    matches!(mv, Move::Drop { kind: k, to } if *k == kind && *to == (r, c))
                }) {
                    self.commit_move(drop_mv);
                    return;
                }
            }
            Selection::None => {}
        }

        if let Some(piece) = self.state.grid[r][c] {
            if piece.owner == self.state.turn {
                self.selection = Selection::BoardSquare(r, c);
                return;
            }
        }
        self.selection = Selection::None;
    }

    fn enforce_three_color_theme(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        let v = &mut style.visuals;

        v.dark_mode = true;
        v.override_text_color = Some(WARM_CREAM);
        v.panel_fill = DEEP_CHOCOLATE;
        v.window_fill = DEEP_CHOCOLATE;
        v.extreme_bg_color = DEEP_CHOCOLATE;
        v.faint_bg_color = SOFT_SAGE;
        v.window_stroke = Stroke::new(2.0_f32, SOFT_SAGE);

        v.widgets.noninteractive.bg_fill = DEEP_CHOCOLATE;
        v.widgets.noninteractive.weak_bg_fill = DEEP_CHOCOLATE;
        v.widgets.noninteractive.bg_stroke = Stroke::new(1.5_f32, SOFT_SAGE);
        v.widgets.noninteractive.fg_stroke = Stroke::new(1.5_f32, WARM_CREAM);

        v.widgets.inactive.bg_fill = SOFT_SAGE;
        v.widgets.inactive.weak_bg_fill = SOFT_SAGE;
        v.widgets.inactive.bg_stroke = Stroke::new(1.5_f32, WARM_CREAM);
        v.widgets.inactive.fg_stroke = Stroke::new(2.0_f32, DEEP_CHOCOLATE);

        v.widgets.hovered.bg_fill = WARM_CREAM;
        v.widgets.hovered.weak_bg_fill = WARM_CREAM;
        v.widgets.hovered.bg_stroke = Stroke::new(2.0_f32, SOFT_SAGE);
        v.widgets.hovered.fg_stroke = Stroke::new(2.0_f32, DEEP_CHOCOLATE);

        v.widgets.active.bg_fill = WARM_CREAM;
        v.widgets.active.weak_bg_fill = WARM_CREAM;
        v.widgets.active.bg_stroke = Stroke::new(2.5_f32, DEEP_CHOCOLATE);
        v.widgets.active.fg_stroke = Stroke::new(2.5_f32, DEEP_CHOCOLATE);

        v.selection.bg_fill = SOFT_SAGE;
        v.selection.stroke = Stroke::new(1.5_f32, DEEP_CHOCOLATE);

        ctx.set_style(style);
    }

    fn draw_bordered_rect(
        painter: &egui::Painter,
        rect: Rect,
        fill: Color32,
        border_color: Color32,
        border_width: f32,
        rounding: f32,
    ) {
        painter.rect_filled(rect, Rounding::same(rounding), border_color);
        let inner = rect.shrink(border_width);
        if inner.width() > 0.0 && inner.height() > 0.0 {
            painter.rect_filled(
                inner,
                Rounding::same((rounding - border_width).max(0.0)),
                fill,
            );
        }
    }

    /// Draws an authentic pentagonal Shogi piece (Koma) with directional movement dots
    fn draw_koma(
        painter: &egui::Painter,
        center: Pos2,
        size: f32,
        piece: Piece,
        is_selected: bool,
    ) {
        let dir = if piece.owner == Player::Sente {
            1.0
        } else {
            -1.0
        };
        let hw = size * 0.41;
        let hh = size * 0.44;
        let shoulder_w = size * 0.33;
        let shoulder_y = -hh * 0.36;

        let points = vec![
            Pos2::new(center.x, center.y - dir * hh),
            Pos2::new(center.x + shoulder_w, center.y + dir * shoulder_y),
            Pos2::new(center.x + hw, center.y + dir * hh),
            Pos2::new(center.x - hw, center.y + dir * hh),
            Pos2::new(center.x - shoulder_w, center.y + dir * shoulder_y),
        ];

        let (body_fill, border_col, text_col, badge_fill) = if is_selected {
            (DEEP_CHOCOLATE, WARM_CREAM, WARM_CREAM, SOFT_SAGE)
        } else if piece.kind.is_promoted() {
            (SOFT_SAGE, DEEP_CHOCOLATE, DEEP_CHOCOLATE, WARM_CREAM)
        } else {
            (WARM_CREAM, DEEP_CHOCOLATE, DEEP_CHOCOLATE, SOFT_SAGE)
        };

        painter.add(Shape::convex_polygon(
            points,
            body_fill,
            Stroke::new(2.2_f32, border_col),
        ));

        if piece.kind.is_promoted() || piece.kind == PieceKind::King {
            painter.circle_filled(
                Pos2::new(center.x, center.y + dir * (size * 0.05)),
                size * 0.22,
                badge_fill,
            );
        }

        painter.text(
            Pos2::new(center.x, center.y + dir * (size * 0.05)),
            Align2::CENTER_CENTER,
            piece.kind.label(),
            FontId::new(size * 0.33, FontFamily::Proportional),
            text_col,
        );

        Self::draw_move_indicators(painter, center, size, piece, dir, border_col, badge_fill);
    }

    fn draw_move_indicators(
        painter: &egui::Painter,
        center: Pos2,
        size: f32,
        piece: Piece,
        dir: f32,
        dot_col: Color32,
        slide_fill: Color32,
    ) {
        let radius = size * 0.28;
        let dirs: &[((f32, f32), bool)] = match piece.kind {
            PieceKind::King => &[
                ((0.0, -1.0), false),
                ((-1.0, -1.0), false),
                ((1.0, -1.0), false),
                ((-1.0, 0.0), false),
                ((1.0, 0.0), false),
                ((-1.0, 1.0), false),
                ((0.0, 1.0), false),
                ((1.0, 1.0), false),
            ],
            PieceKind::Gold
            | PieceKind::PromotedSilver
            | PieceKind::PromotedKnight
            | PieceKind::PromotedLance
            | PieceKind::PromotedPawn => &[
                ((0.0, -1.0), false),
                ((-1.0, -1.0), false),
                ((1.0, -1.0), false),
                ((-1.0, 0.0), false),
                ((1.0, 0.0), false),
                ((0.0, 1.0), false),
            ],
            PieceKind::Silver => &[
                ((0.0, -1.0), false),
                ((-1.0, -1.0), false),
                ((1.0, -1.0), false),
                ((-1.0, 1.0), false),
                ((1.0, 1.0), false),
            ],
            PieceKind::Knight => &[((-0.7, -1.0), false), ((0.7, -1.0), false)],
            PieceKind::Lance => &[((0.0, -1.0), true)],
            PieceKind::Pawn => &[((0.0, -1.0), false)],
            PieceKind::Rook => &[
                ((0.0, -1.0), true),
                ((0.0, 1.0), true),
                ((-1.0, 0.0), true),
                ((1.0, 0.0), true),
            ],
            PieceKind::Bishop => &[
                ((-1.0, -1.0), true),
                ((1.0, -1.0), true),
                ((-1.0, 1.0), true),
                ((1.0, 1.0), true),
            ],
            PieceKind::PromotedRook => &[
                ((0.0, -1.0), true),
                ((0.0, 1.0), true),
                ((-1.0, 0.0), true),
                ((1.0, 0.0), true),
                ((-1.0, -1.0), false),
                ((1.0, -1.0), false),
                ((-1.0, 1.0), false),
                ((1.0, 1.0), false),
            ],
            PieceKind::PromotedBishop => &[
                ((-1.0, -1.0), true),
                ((1.0, -1.0), true),
                ((-1.0, 1.0), true),
                ((1.0, 1.0), true),
                ((0.0, -1.0), false),
                ((0.0, 1.0), false),
                ((-1.0, 0.0), false),
                ((1.0, 0.0), false),
            ],
        };

        for &((dx, dy), is_slide) in dirs {
            let pos = Pos2::new(
                center.x + dx * radius * 0.78,
                center.y + dy * dir * radius * 0.78 + dir * (size * 0.04),
            );
            if is_slide {
                painter.circle_filled(pos, size * 0.045, slide_fill);
                painter.circle_stroke(pos, size * 0.045, Stroke::new(1.2_f32, dot_col));
            } else {
                painter.circle_filled(pos, size * 0.032, dot_col);
            }
        }
    }

    fn render_komadai(&mut self, ui: &mut egui::Ui, owner: Player) {
        let is_active_turn = self.state.turn == owner
            && (!self.vs_ai || owner != self.ai_player)
            && !self.game_over;

        Self::draw_bordered_rect(
            ui.painter(),
            ui.available_rect_before_wrap(),
            DEEP_CHOCOLATE,
            SOFT_SAGE,
            1.5,
            6.0,
        );

        ui.add_space(6.0);
        ui.vertical_centered(|ui| {
            let header = if self.state.turn == owner {
                format!("▶ {} Hand (Komadai)", owner.name())
            } else {
                format!("{} Hand (Komadai)", owner.name())
            };
            ui.label(
                egui::RichText::new(header)
                    .strong()
                    .size(14.5)
                    .color(WARM_CREAM),
            );
        });
        ui.add_space(6.0);

        let hand = *self.state.hand_ref(owner);
        ui.horizontal_wrapped(|ui| {
            ui.add_space(6.0);
            for (idx, &kind) in HAND_KINDS.iter().enumerate() {
                let count = hand[idx];
                let selected = is_active_turn && self.selection == Selection::HandPiece(kind);

                let (rect, resp) = ui.allocate_exact_size(Vec2::new(72.0, 56.0), Sense::click());
                let painter = ui.painter();

                let fill = if selected {
                    SOFT_SAGE
                } else if count > 0 {
                    WARM_CREAM
                } else {
                    DEEP_CHOCOLATE
                };
                let border = if selected {
                    WARM_CREAM
                } else {
                    SOFT_SAGE
                };
                let text_col = if count > 0 || selected {
                    DEEP_CHOCOLATE
                } else {
                    SOFT_SAGE
                };

                Self::draw_bordered_rect(painter, rect, fill, border, 1.5, 5.0);

                painter.text(
                    Pos2::new(rect.center().x, rect.top() + 18.0),
                    Align2::CENTER_CENTER,
                    format!("{} ×{}", kind.label(), count),
                    FontId::new(15.0, FontFamily::Proportional),
                    text_col,
                );
                painter.text(
                    Pos2::new(rect.center().x, rect.bottom() - 14.0),
                    Align2::CENTER_CENTER,
                    kind.title(),
                    FontId::new(9.5, FontFamily::Proportional),
                    text_col,
                );

                if resp.clicked() && is_active_turn && count > 0 {
                    if self.selection == Selection::HandPiece(kind) {
                        self.selection = Selection::None;
                    } else {
                        self.selection = Selection::HandPiece(kind);
                    }
                }
            }
        });
        ui.add_space(6.0);
    }
}

impl eframe::App for ShogiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Self::enforce_three_color_theme(ctx);

        if !self.game_over
            && self.pending_promotion.is_none()
            && self.vs_ai
            && self.state.turn == self.ai_player
        {
            if let Some(ai_mv) = self.state.choose_ai_move(2) {
                self.commit_move(ai_mv);
            }
        }

        let legal_moves = self.state.generate_legal_moves();

        let active_targets: Vec<(usize, usize)> = match self.selection {
            Selection::BoardSquare(sr, sc) => legal_moves
                .iter()
                .filter_map(|mv| match *mv {
                    Move::Board { from, to, .. } if from == (sr, sc) => Some(to),
                    _ => None,
                })
                .collect(),
            Selection::HandPiece(kind) => legal_moves
                .iter()
                .filter_map(|mv| match *mv {
                    Move::Drop { kind: k, to } if k == kind => Some(to),
                    _ => None,
                })
                .collect(),
            Selection::None => Vec::new(),
        };

        egui::SidePanel::right("shogi_control_panel")
            .exact_width(335.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("SHOGI • 将棋")
                            .size(24.0)
                            .strong()
                            .color(WARM_CREAM),
                    );
                    ui.label(
                        egui::RichText::new("Chocolate • Soft Sage • Warm Cream")
                            .size(12.0)
                            .color(SOFT_SAGE),
                    );
                });
                ui.add_space(8.0);

                let (banner_rect, _) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width(), 44.0),
                    Sense::hover(),
                );
                let banner_fill = if self.game_over || self.state.is_in_check(self.state.turn) {
                    SOFT_SAGE
                } else {
                    WARM_CREAM
                };
                Self::draw_bordered_rect(
                    ui.painter(),
                    banner_rect,
                    banner_fill,
                    DEEP_CHOCOLATE,
                    2.0,
                    6.0,
                );
                ui.painter().text(
                    banner_rect.center(),
                    Align2::CENTER_CENTER,
                    &self.status_banner,
                    FontId::new(14.0, FontFamily::Proportional),
                    DEEP_CHOCOLATE,
                );

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("New Game").clicked() {
                        self.reset_game();
                    }
                    if ui.button("Undo Move").clicked() {
                        self.undo_move();
                    }
                    let mode_label = if self.vs_ai {
                        "Mode: 1P vs AI"
                    } else {
                        "Mode: 2P Local"
                    };
                    if ui.button(mode_label).clicked() {
                        self.vs_ai = !self.vs_ai;
                        self.selection = Selection::None;
                    }
                });

                ui.add_space(10.0);
                self.render_komadai(ui, Player::Gote);
                ui.add_space(10.0);
                self.render_komadai(ui, Player::Sente);
                ui.add_space(10.0);

                ui.label(
                    egui::RichText::new("Move Log (Kifu)")
                        .strong()
                        .size(14.0)
                        .color(SOFT_SAGE),
                );
                egui::ScrollArea::vertical()
                    .max_height(180.0)
                    .show(ui, |ui| {
                        for entry in self.move_log.iter().rev() {
                            ui.label(
                                egui::RichText::new(entry)
                                    .size(13.0)
                                    .color(WARM_CREAM),
                            );
                        }
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let avail = ui.available_rect_before_wrap();
            let board_dim = avail.width().min(avail.height()) - 36.0;
            let cell = (board_dim / 9.4).max(38.0);
            let grid_span = cell * 9.0;

            let board_rect = Rect::from_center_size(avail.center(), Vec2::splat(grid_span));
            let outer_frame = board_rect.expand(22.0);
            let painter = ui.painter();

            Self::draw_bordered_rect(
                painter,
                outer_frame,
                WARM_CREAM,
                SOFT_SAGE,
                3.5,
                8.0,
            );

            for i in 0..9 {
                let file_lbl = ((b'9' - i as u8) as char).to_string();
                let fx = board_rect.left() + (i as f32 + 0.5) * cell;
                painter.text(
                    Pos2::new(fx, board_rect.top() - 11.0),
                    Align2::CENTER_CENTER,
                    file_lbl,
                    FontId::new(13.0, FontFamily::Proportional),
                    DEEP_CHOCOLATE,
                );

                let rank_lbl = ((b'a' + i as u8) as char).to_string();
                let ry = board_rect.top() + (i as f32 + 0.5) * cell;
                painter.text(
                    Pos2::new(board_rect.right() + 11.0, ry),
                    Align2::CENTER_CENTER,
                    rank_lbl,
                    FontId::new(13.0, FontFamily::Proportional),
                    DEEP_CHOCOLATE,
                );
            }

            let mut clicked_square: Option<(usize, usize)> = None;

            for r in 0..9 {
                for c in 0..9 {
                    let sq_min = Pos2::new(
                        board_rect.left() + c as f32 * cell,
                        board_rect.top() + r as f32 * cell,
                    );
                    let sq_rect = Rect::from_min_size(sq_min, Vec2::splat(cell));
                    let resp = ui.interact(
                        sq_rect,
                        egui::Id::new(("shogi_sq", r, c)),
                        Sense::click(),
                    );
                    if resp.clicked() {
                        clicked_square = Some((r, c));
                    }

                    let is_selected = self.selection == Selection::BoardSquare(r, c);
                    let is_last_move = self
                        .state
                        .last_move
                        .map_or(false, |mv| mv.to_square() == (r, c));

                    if is_selected || is_last_move {
                        painter.rect_filled(sq_rect, Rounding::ZERO, SOFT_SAGE);
                    } else {
                        painter.rect_filled(sq_rect, Rounding::ZERO, WARM_CREAM);
                    }

                    painter.line_segment(
                        [sq_rect.left_top(), sq_rect.right_top()],
                        Stroke::new(1.5_f32, DEEP_CHOCOLATE),
                    );
                    painter.line_segment(
                        [sq_rect.left_top(), sq_rect.left_bottom()],
                        Stroke::new(1.5_f32, DEEP_CHOCOLATE),
                    );

                    if let Some(piece) = self.state.grid[r][c] {
                        Self::draw_koma(
                            painter,
                            sq_rect.center(),
                            cell * 0.88,
                            piece,
                            is_selected,
                        );
                    }

                    if active_targets.contains(&(r, c)) {
                        if self.state.grid[r][c].is_some() {
                            painter.circle_stroke(
                                sq_rect.center(),
                                cell * 0.41,
                                Stroke::new(3.5_f32, SOFT_SAGE),
                            );
                            painter.circle_stroke(
                                sq_rect.center(),
                                cell * 0.44,
                                Stroke::new(1.5_f32, DEEP_CHOCOLATE),
                            );
                        } else {
                            painter.circle_filled(sq_rect.center(), cell * 0.16, SOFT_SAGE);
                            painter.circle_stroke(
                                sq_rect.center(),
                                cell * 0.16,
                                Stroke::new(1.8_f32, DEEP_CHOCOLATE),
                            );
                        }
                    }
                }
            }

            painter.line_segment(
                [board_rect.right_top(), board_rect.right_bottom()],
                Stroke::new(2.0_f32, DEEP_CHOCOLATE),
            );
            painter.line_segment(
                [board_rect.left_bottom(), board_rect.right_bottom()],
                Stroke::new(2.0_f32, DEEP_CHOCOLATE),
            );

            for &hr in &[3usize, 6usize] {
                for &hc in &[3usize, 6usize] {
                    let pt = Pos2::new(
                        board_rect.left() + hc as f32 * cell,
                        board_rect.top() + hr as f32 * cell,
                    );
                    painter.circle_filled(pt, 4.0, DEEP_CHOCOLATE);
                }
            }

            if let Some((r, c)) = clicked_square {
                self.handle_square_click(r, c, &legal_moves);
            }
        });

        if let Some(promo) = &self.pending_promotion {
            let from = promo.from;
            let to = promo.to;
            let mut decision: Option<bool> = None;

            egui::Window::new("Promote Piece? (成 / 不成)")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(
                            "Your piece entered or moved within the promotion zone.",
                        )
                            .color(WARM_CREAM),
                    );
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Promote (成)").clicked() {
                            decision = Some(true);
                        }
                        if ui.button("Stay Unpromoted (不成)").clicked() {
                            decision = Some(false);
                        }
                    });
                    ui.add_space(6.0);
                });

            if let Some(promote) = decision {
                self.commit_move(Move::Board { from, to, promote });
            }
        }
    }
}

fn setup_japanese_font(ctx: &egui::Context) {
    if let Ok(font_bytes) = std::fs::read("C:\\Windows\\Fonts\\msgothic.ttc") {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "ms_gothic".to_owned(),
            egui::FontData::from_owned(font_bytes).into(),
        );
        if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
            family.push("ms_gothic".to_owned());
        }
        ctx.set_fonts(fonts);
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Shogi (将棋) — Deep Chocolate, Soft Sage & Warm Cream Edition")
            .with_inner_size([1060.0, 760.0])
            .with_min_inner_size([860.0, 620.0]),
        ..Default::default()
    };

    eframe::run_native(
        "shogi_rust",
        native_options,
        Box::new(|cc| {
            setup_japanese_font(&cc.egui_ctx);
            Ok(Box::new(ShogiApp::default()))
        }),
    )
}