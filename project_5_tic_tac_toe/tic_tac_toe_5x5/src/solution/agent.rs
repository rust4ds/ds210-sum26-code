use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

//heuristic aka evaluation function - determines score of current game state (not at the end)
fn heuristic(board: &Board) -> i32 {
    let cells = board.get_cells(); //gets current posiiton of cells
    let size = cells.len();  //determines if 3x3 or 5x5
    let mut score: i32 = 0; 

    //check for actual winning lines > have to adjust the weights so that it can pass test
    let real_score = board.score();
    if real_score != 0 {
        return real_score * 100; 
    }
    //helper function that checks each cell
    fn count_cell (line: [&Cell; 3], target: &Cell) -> usize {
        let mut count = 0;
        for cell in line {
            if cell == target {
                count += 1;
            }
        }
        count
    }

    //helper function that sets up scores each line 
    fn score_line(a: &Cell, b:&Cell, c:&Cell) -> i32 {
        let line = [a, b, c]; //initialize what a line is
        let x_count = count_cell(line, &Cell::X);
        let o_count = count_cell(line, &Cell::O);
        let wall_count = count_cell(line, &Cell::Wall);

        //score is 0 if there is wall, or there is both x and o in the same line
        if wall_count > 0 || (x_count > 0 && o_count > 0) {
            return 0;
        }
        //scores for open lines of X:
        if o_count == 0 {
            return match x_count {
                2 => 10, //if 2 X's, then +10 points
                1 => 1, //if 1 X, then +1 
                _ => 0,
            };
        }
        //scores for open lines of O:
        if x_count == 0 {
            return match o_count {
                2 => -10, //reverse
                1 => -1,
                _ => 0,
            };
        }
        0
    }

    //now scan each possible 3 cell window (diagonal, horizontal and vertical)
    for i in 0..size {
        for j in 0..size {
            //rows 
            if j+2 <size {
                score += score_line(&cells[i][j], &cells[i][j+1], &cells[i][j+2]);
            }
            //column
            if i+ 2 < size{
                score += score_line(&cells[i][j], &cells[i+1][j], &cells[i+2][j]);
            }
            //diagonal -ve slope 
            if i + 2 < size && j +2 <size {
                score += score_line(&cells[i][j], &cells[i+1][j+1], &cells[i+2][j+2]);
            }
            //diagonal +ve slope
            if i + 2 < size && j >= 2 {
                score += score_line(&cells[i][j], &cells[i+1][j-1], &cells[i+2][j-2]);
            }
        }
    }
    //now give bonus points to if they are in the center
    let mid = size/2 ; 
    for i in 0..size {
        for j in 0..size {
            let dist = (( i as i32 - mid as i32).abs()).max((j as i32 - mid as i32).abs());
            let center_bonus = match dist {
                0 => 2, //if they are in center> +2 
                1 => 1,
                _ => 0,
            };
            match &cells[i][j] {
                Cell::X => score += center_bonus,
                Cell::O => score -= center_bonus,
                _ => {}
            }
        }
    }

    score
}

//minimax helper function with alpha beta pruning
//alpha = best x score 
//beta = best o score 
fn minimax (board: &mut Board, player: Player, depth: u32, mut alpha: i32, mut beta: i32,) -> (i32, usize, usize) {
    //check if game is over
    if board.game_over() {
        return (board.score(), 0,0);
    }

    //if depth reaches 0 > get game state score using heuristic function
    if depth == 0 {
        return (heuristic(board), 0, 0);
    }
    //same as 3x3 
    let moves = board.moves();
    let mut best_score = match player {
        Player::X => i32::MIN,
        Player::O => i32::MAX,
    };
    let mut best_move = moves[0];
    let next_player = match player {
        Player::X => Player::O,
        Player::O => Player::X,
    };
    for m in moves {
        board.apply_move(m,player);
        let (score, _, _) = minimax(board, next_player, depth -1, alpha, beta);
        board.undo_move(m,player);

        //check if this is better move
        let better_move = match player {
            Player::X => score > best_score,
            Player::O => score < best_score,
        };

        if better_move {
            best_score = score;
            best_move = m;
        }

        //alpha-beta pruning = if beta < alpha then break don't explore the current m move
        match player {
            Player::X => {
                if best_score > alpha {alpha = best_score;} 
                if beta <= alpha {break;}
            }
            Player::O => {
                if best_score < beta {beta = best_score;}
                if beta <= alpha {break;}
            }
        }
    }
    (best_score, best_move.0, best_move.1)

}
// Put your solution here.
impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let size = board.get_cells().len();
        let depth = if size <= 3 {9} else {5};

        minimax(board, player, depth, i32::MIN, i32::MAX)
    }
}
    


