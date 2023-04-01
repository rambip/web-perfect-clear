use yew::prelude::*;
use web_sys::{window, HtmlInputElement};


#[derive(PartialEq, Clone, Copy)]
enum Couleur {
    T=0,
    S=1,
    Z=2,
    O=3,
    I=4,
    L=5,
    J=6,
}

impl Couleur {
    fn paint(&self) -> &'static str {
        match self {
            Couleur::T => "purple",
            Couleur::S => "green",
            Couleur::Z => "red",
            Couleur::O => "yellow",
            Couleur::I => "cyan",
            Couleur::L => "orange",
            Couleur::J => "blue",
        }
    }
}

struct EtatJeu {
    grille: [[bool; 10]; 4],
    pieces_jouees: Vec<u8>,
    // TODO: retirer
    couleur_jouees: [u8; 7],
}




const N_PIECES: usize = 19;
// Chaque piece existe en plusieurs versions selon l'orientation
// - position relative des 3 blocs adjacents
// - couleur
static PIECES: [([(i32, i32); 3], Couleur); N_PIECES] = [
    ([( 0,  1), (-1,  1), (-1,  2)], Couleur::S), // S
    ([( 1,  0), ( 1,  1), ( 2,  1)], Couleur::S), // S'
    ([( 0,  1), ( 1,  1), ( 1,  2)], Couleur::Z), // Z
    ([( 0,  1), ( 1,  0), (-1,  1)], Couleur::Z), // Z'
    ([( 0,  1), ( 0,  2), ( 1,  1)], Couleur::T), // T1
    ([( 1,  0), ( 1,  1), ( 2,  0)], Couleur::T), // T2
    ([( 0,  1), (-1,  1), ( 0,  2)], Couleur::T), // T3
    ([(-1,  1), ( 0,  1), ( 1,  1)], Couleur::T), // T4
    ([( 0,  1), (-1,  1), (-2,  1)], Couleur::J), // J1
    ([( 0,  1), ( 0,  2), ( 1,  2)], Couleur::J), // J2
    ([( 1,  0), ( 0,  1), ( 2,  0)], Couleur::J), // J3
    ([( 1,  0), ( 1,  1), ( 1,  2)], Couleur::J), // J4
    ([( 1,  0), ( 2,  0), ( 2,  1)], Couleur::L), // L1
    ([( 0,  1), ( 0,  2), (-1,  2)], Couleur::L), // L2
    ([( 0,  1), ( 1,  1), ( 2,  1)], Couleur::L), // L3
    ([( 0,  1), ( 0,  2), ( 1,  0)], Couleur::L), // L4
    ([( 1,  0), ( 2,  0), ( 3,  0)], Couleur::I), // I
    ([( 0,  1), ( 0,  2), ( 0,  3)], Couleur::I), // I'
    ([( 0,  1), ( 1,  0), ( 1,  1)], Couleur::O),
];


#[derive(Copy, Clone, Debug)]
struct Coup {
    // position sur la grille
    i: i32,
    j: i32,
    // la piece posee est un index dans PIECES
    piece: usize,
}


impl Coup {
    // donne les 4 blocs touches sur le plateau par le coup indique
    // si le coup n'est pas valide, les blocs peuvent sortir de la grille !
    fn blocs_touches(&self) -> [(i32, i32); 4] {
        let (blocs, _couleur) = &PIECES[self.piece];

        let mut result = [(0, 0); 4];
        result[0] = (self.i, self.j);

        for i in 0..=2 {
            result[i+1] = (
                (self.i + blocs[i].0),
                (self.j + blocs[i].1) 
            );
        };
        
        result
    }
}


fn case_vide(g: [[bool; 10]; 4]) -> (usize, usize) {
    for j in 0..10 {
        for i in 0..4 {
            if g[i][j] == false {
                return (i, j);
            }
        }
    }
    panic!("pas de case vide !!!");
}

/// Prend une grille de blocs, un coup et indique si le coup est valide pour ce plateau de jeu.
fn valide(etat: &EtatJeu, coup: Coup) -> bool {
    // pour chaque bloc de la piece, on regarde a quel endroit elle atterit sur la grille.
    // si la piece sort de la grille ou qu'il y a une collision, le coup est invalide
    for (i, j) in coup.blocs_touches() {
        if i < 0 || i>=4 || j < 0 || j>=10 {return false};

        if etat.grille[i as usize][j as usize] {return false};

    };

    true
}


fn jouer_coup(etat: &mut EtatJeu, coup: Coup) {
    let couleur = PIECES[coup.piece].1;
    for (i, j) in coup.blocs_touches() {
        etat.grille[i as usize][j as usize] = true;
    }
    etat.pieces_jouees.push(coup.piece as u8);
    etat.couleur_jouees[couleur as usize]+=1;
}

fn annuler_coup(etat: &mut EtatJeu, coup: Coup) {
    let couleur = PIECES[coup.piece].1;
    for (i,j) in coup.blocs_touches() {
        etat.grille[i as usize][j as usize] = false;
    }
    etat.pieces_jouees.pop();
    etat.couleur_jouees[couleur as usize]-=1;

}


// donne la liste de tous les coups pouvant etre joues a ce tour:
fn coup_valides(etat: &EtatJeu) -> Vec<Coup> {
    // on joue sur la prochaine case libre:
    let mut r = Vec::with_capacity(N_PIECES);
    let (i, j) = case_vide(etat.grille);
    for piece in 0..N_PIECES {
        let coup = Coup {i:i as i32, j:j as i32, piece};
        if valide(etat, coup){
            r.push(coup);
        }
    }
    r
}


fn liste_perfect_clear(etat: &mut EtatJeu, acc: &mut Vec<Vec<u8>>) {
    if etat.pieces_jouees.len() == 10 {
        if etat.couleur_jouees.iter().all(|&x| x>=1 && x <= 2) {
            // si toutes les couleurs de pieces on ete posees
            acc.push(etat.pieces_jouees.clone());
        }
        return;
    }

    for coup in coup_valides(etat) {
        jouer_coup(etat, coup);

        liste_perfect_clear(etat, acc);

        annuler_coup(etat, coup);
    }
}

struct Model {
    possibilities : Option<Vec<Vec<u8>>>,
    n: usize,
    width: u32,
    height: u32,
    size_bloc: i32,
}


enum Msg {
    NextPossibility,
    PrevPossibility,
    Compute,
    JumpToPossibility(f32),
}


impl Model {
    fn display_tetris(&self) -> Html {
        match &self.possibilities {
            None => html!(),
            Some(l) => {
                // genere la grille a partir des indexes des pieces posees
                let mut grille = [[false; 10]; 4];

                // indexe des pieces 
                let mut grille_pieces = [[None; 12]; 6];

                // liste de composants svg (rectangles)
                let mut shapes = Vec::new();

                for (num, &i_piece) in l[self.n].iter().enumerate() {

                    // reconstruit le coup suivant
                    let (i, j) = case_vide(grille);
                    let coup : Coup = Coup { i: i as i32, j: j as i32, piece: i_piece as usize};

                    // affiche les differents blocs de ce tetramino
                    for (i, j) in coup.blocs_touches().iter() {
                        let (i, j) = (*i as usize, *j as usize);
                        grille[i][j] = true;
                        grille_pieces[i+1][j+1] = Some(num);

                        let x = 50+self.size_bloc*(j as i32);
                        let y = 50+self.size_bloc*(i as i32);

                        let color = PIECES[i_piece as usize].1.paint();

                        shapes.push(
                            html!{
                                <rect 
                                    x={x.to_string()} 
                                    y={y.to_string()} 
                                    width={self.size_bloc.to_string()} 
                                    height={self.size_bloc.to_string()} 
                                    fill={color}/>
                            }
                        );


                        // indique si la case (i', j') adjacente a besoin d'une bordure:
                        // seulement si elle est issue d'un tetramino different
                        let contour = | i_ad : usize, j_ad: usize| match grille_pieces[i_ad+1][j_ad+1] {
                            None => false,
                            Some(n) => n != num,
                        };

                        // traduit deux coordonées en ligne SVG
                        let line = |x1: i32, y1: i32, x2: i32, y2: i32| html!{
                                    <line 
                                        x1={x1.to_string()} 
                                        y1={y1.to_string()} 
                                        x2={x2.to_string()} 
                                        y2={y2.to_string()} 
                                        stroke-width={2}
                                        stroke={"white"}/>
                        };


                        if contour(i-1, j) {
                            shapes.push(line(x, y, x+self.size_bloc, y));
                        }
                        if contour(i+1, j) {
                            shapes.push(line(x, y+self.size_bloc, x+self.size_bloc, y+self.size_bloc));
                        }
                        if contour(i, j-1) {
                            shapes.push(line(x, y, x, y+self.size_bloc));
                        }
                        if contour(i, j+1) {
                            shapes.push(line(x+self.size_bloc, y, x+self.size_bloc, y+self.size_bloc));
                        }
                    }
                }
                shapes.into_iter().collect::<Html>()
            }
        }
    }
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let raw_height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;
        let raw_width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;

        Self {
            possibilities: None,
            n: 0,
            width: raw_width - 200,
            height: raw_height - 300,
            size_bloc: ((u32::min(raw_height, raw_width) - 100)/12) as i32
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match (msg, &self.possibilities) {
            (Msg::NextPossibility, Some(pos)) =>  {
                self.n = (self.n+1) % pos.len();
            },
            (Msg::PrevPossibility, Some(pos)) =>  {
                self.n = (self.n-1) % pos.len();
            },
            (Msg::Compute, _) => {
                let mut possibilities = Vec::new();

                let mut etat_vide: EtatJeu = EtatJeu {
                    grille: [[false; 10]; 4],
                    pieces_jouees: Vec::new(),
                    couleur_jouees: [0; 7],
                };

                liste_perfect_clear(&mut etat_vide, &mut possibilities);
                assert_ne!(possibilities.len(), 0);
                self.possibilities = Some(possibilities);
            },
            (Msg::JumpToPossibility(x), Some(pos)) => {
                let i: f32 = x*((pos.len()-1) as f32);
                self.n = i as usize;
            }
            (_, None) => ()
        }
        true
    }



    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let slider_value = |e: InputEvent| 
            e.target_unchecked_into::<HtmlInputElement>()
            .value_as_number() as f32;


        match &self.possibilities {
            None => {
                html!{
                    <button onclick={link.callback(|_| Msg::Compute)}>{"Calculer"}</button>
                }
            },
            Some(pos) => {
                let n_pos = pos.len();
                html! {
                    <div>
                        <svg width={self.width.to_string()} height={self.height.to_string()}>
                        {self.display_tetris()}
                    </svg>
                        <div>{format!("combinaison {} / {}", self.n, pos.len())}</div>
                        <button onclick={link.callback(|_| Msg::PrevPossibility)}>{"precedent"}</button>
                        <input oninput={link.callback(move |e| Msg::JumpToPossibility(slider_value(e)))} type="range" min="0" step="0.0005" max="1" value={(self.n as f32 / n_pos as f32).to_string()} class="slider" style={format!("width:{}px", self.width/2).to_string()}/>
                        <button onclick={link.callback(|_| Msg::NextPossibility)}>{"suivant"}</button>

            </div>
        }
            }
        }

    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

fn main() {
    //wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
