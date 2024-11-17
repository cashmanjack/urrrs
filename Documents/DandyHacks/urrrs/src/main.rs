use bevy::asset::io::file;
// //! This example illustrates how to create a button that changes color and text based on its
// //! interaction state.
/*
Algorithms
Automata - Seiferas Joel
Artificial intelligence
Additive combinatorics - Hosseini Kaave
AI accelerators - Geng Tong (Tony)
Augmented/Virtual Reality
Assistive Technology - Bai Zhen
Audio Signal Processing - Duan Zhiyao
Agent communication languages/architectures - Ferguson George
Attention - Iordan Coraline
Applied Machine Learning - Kanan Christopher

Brain-computer/machine interfaces - Cetin Mujdat
Biomedical sensing/imaging/analysis - Cetin Mujdat
Biomedical Informatics - Luo Jiebo

Cryptography - Venkitasubramaniam, Muthuramakrishnan
Computer Architecture
Complexity Theory
Combinatorics - Stefankovic Daniel
Computational Social Choice - Kahng Anson
Computer Security - Criswell John
Compilers - Pai Sreepathi
Cloud computing - Heinzelman Wendi
Computer Imaging and Graphics - Zhu Yuhao
Computational Art - Zhu Yuhao
Computer-Supported Collaborative Work - Bai Zhen
Common-sense reasoning/planning
Computer Audition - Duan Zhiyao
Cognitive/Perceptual Learning - Jacobs Robert
Cognitive Science - Kahng Anson
Computer Vision
Computational Social Science - Kang Jian
Computational Linguistics - White Aaron

Democracy in CS - Kahng Anson
Data management/science
Data mining - Kang Jian
Data discovery/integration - Nargesian Fatemeh
Depth perception/processing of binocular disparity - Haefner Ralf
Data science - Kautz Henry

Economics and computation - Kahng Anson
Energy-efficient computer microarchitecture - Huang Michael
Embodied Conversational Agent - Bai Zhen

Theoretical Computer Science
Transactional memory - Scott Michael
Tangible User Interfaced - Bai Zhen
Technology-Enhanced Learning - Bai Zhen
Theoretical condensed-matter physics - Ghoshal Gourab
Trustworthy AI - Kang Jian

User interfaces - Ferguson George
Ubiquitous/Mobile Computing - Luo Jiebo

Graph Theory - Stefankovic Daniel
Game theory - Stefankovic Daniel
GPU systems/architectures - Yanan Guo
GPU algorithms - Pai Sreepathi

Fourier transform and Discrete Fourier analysis

Interactive machine learning - Hoque Ehsan

Networks - Dwarkadas Sandhya
Non-von Neuman computing - Huang Michael
Natural language understanding/processing/semantics
Neural population encoding/decoding - Haefner Ralf
Neuroplasticity - Iordan Coraline
Neural networks - Iordan Coraline
Neural Computation - Jacobs Robert
Neumerical programming - Purtee Adam

Marov chains/counting - Stefankovic Daniel
Memory management/sharing
Machine learning
Mobile ad hoc networks - Heinzelman Wendi
Multimedia communication - Heinzelman Wendi
Music Information Retrieval - Duan Zhiyao
Machine translation - Gildea Daniel
Motion Planning - Howard Thomas
Multimodal Modeling - Xu Chenliang

Learning theory - Stefankovic Daniel
Locality theory and optimizations - Ding Chen
Language and dialogue - Schubert Lenhart

Wireless sensor networks - Heinzelman Wendi

Knowledge representation

Phylogeny - Stefankovic Daniel
Pseudorandomness - Hosseini Kaave
Parallel and Distributed Systems - Scott Michael
Performance evaluation/modeling
Programming languages - Scott Michael
Persistent memory - Scott Michael
Perceptual decision-making - Haefner Ralf
Probabilistic inference - Haefner Ralf
Pervasive computing - Kautz Henry

High-performance Computing - Huang Michael
Energy-efficient computing microarchitecture - Huang Michael
Heterogeneous networking - Heinzelman Wendi
Heterogeneous Architectures - Pai Sreepathi
Human Perception and Cognition - Zhu Yuhao
Human-Computer Interaction
Health and wellbeing - Hoque Ehsan
Human Behavior Modeling - Yan Yukang
Human-Robot Interaction - Howard Thomas

Secure Virtual Architecture - Criswell John
Synchronization - Scott Michael
Speech recognition - Ferguson George
Semantic web - Ferguson George
Semantic parsing - Gildea Daniel
Sampling - Liang Jiaming
Social Media - Luo Jiebo
Schema-based learning/behavior - Schubert Lenhart
Self-motivated agents - Schubert Lenhart

Routing - Stefankovic Daniel
Reasoning
Robotics - Howard Thomas
Real-time fMRI - Iordan Coraline

Ising machines - Huang Michael
Intelligen agents - Ferguson George

Visual Semantic Cognition - Iordan Coraline
Video Analysis - Xu Chenliang

Optimization - Liang Jiaming




vec struct
professor main page
interest page
paper1...10


0 = main
1 = Research Overview
2 = Artificial Intelligence
3 = Human-Computer Interaction
4 = Computer Systems
5 = Theory
*/
use bevy::{prelude::*, winit::WinitSettings};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufReader;


// #[derive(Debug, Deserialize)]
// struct Author {
//     first: String,
//     last: String,
// }

// #[derive(Debug, Deserialize)]
// struct Publication {
//     author: Author,
//     title: String,
//     r#abstract: String,
//     url: String,
//     pub_year: i32,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Professor {
    first: String,
    last: String,
    middle: Option<String>,
    website: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publication {
    author: Professor,
    title: String,
    r#abstract: String,
    url: String,
    pub_year: Option<u64>,
}

#[derive(Component, Clone)]
struct ButtonId(usize);

#[derive(Clone)]
struct ButtonData {
    button_bundle: ButtonBundle,
    button_text: String,
    button_id: ButtonId,
}

#[derive(Resource)]
struct ButtonSets(Vec<Vec<ButtonData>>);

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn create_button(text: &str, button_id: ButtonId) -> ButtonData {
    let width = 200.0; // Base width + offset based on text length
    let height = 200.0; // Keep the height constant or adjust similarly

    ButtonData {
        button_bundle: ButtonBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        },
        button_text: text.to_string(),
        button_id,
    }
}

fn create_large_button(text: &str, button_id: ButtonId) -> ButtonData {
    let width = 500.0; // Base width + offset based on text length
    let height = 500.0; // Keep the height constant or adjust similarly

    ButtonData {
        button_bundle: ButtonBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                justify_content: JustifyContent::Stretch,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        },
        button_text: text.to_string(),
        button_id,
    }
}



fn search_by_name(file_path: &str, first_name: &str, last_name: &str) -> Vec<Publication> {
    // Open the file and create a buffered reader
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Parse the entire file as a Vec<Publication>
    let publications: Vec<Publication> = serde_json::from_reader(reader).expect("Error parsing JSON");

    // Filter publications by author's first and last name, sort by year, and take the top 10
    let mut filtered_publications: Vec<Publication> = publications.into_iter()
        .filter(|publication| {
            publication.author.first == first_name && publication.author.last == last_name
        })
        .collect();

    // Sort by publication year in descending order
    filtered_publications.sort_by(|a, b| b.pub_year.cmp(&a.pub_year));

    // Return the top 10 publications
    filtered_publications.into_iter().take(10).collect()
}

// // Create the ButtonSet for a given professor and index
fn create_professor_button_set(publications: Vec<Publication>, mut index: usize, first_name: &str, last_name: &str) -> Vec<Vec<ButtonData>> {
    let mut button_set: Vec<Vec<ButtonData>> = vec![vec![
        create_button("Home", ButtonId(0)),
        create_button(&format!("Professor {} {}", first_name, last_name).to_string(), ButtonId(index))
    ]];

    let prof_index = index;

    for paper in publications.iter().take(10) {
        index += 1;
        button_set[0].push(create_button(&format!("Publication {}", index-prof_index).to_string(), ButtonId(index)));
        let paper_buttons = vec![
            create_button("Back to professor", ButtonId(prof_index)),
            create_large_button(&format!("Title: {}", paper.title).to_string(), ButtonId(index)),
            create_large_button(&format!("Abstract: {}", paper.r#abstract.clone()).to_string(), ButtonId(index)),
            create_button(&format!("Publication Year: {}", paper.pub_year.unwrap_or(2024)).to_string(), ButtonId(index)),
        ];
        button_set.push(paper_buttons);
    }
    button_set
}

fn main() {
    let mut button_set: ButtonSets = ButtonSets(vec![
        vec![
            create_button("Research Interests", ButtonId(1)),
            create_button("Artificial Intelligence", ButtonId(2)),
            create_button("Human-Computer Interaction", ButtonId(3)),
            create_button("Computer Systems", ButtonId(4)),
            create_button("Theory", ButtonId(5)),
        ],
        // Second set of buttons
        vec![
            create_button("Back", ButtonId(0)),
            create_button("Multiple professor interests 1", ButtonId(6)),
            create_button("Multiple professor interests 2", ButtonId(7)),
            create_button("Single professor interests 1", ButtonId(8)),
            create_button("Single professor interests 2", ButtonId(9)),
            create_button("Single professor interests 3", ButtonId(10)),
            create_button("Single professor interests 4", ButtonId(11)),
            create_button("Single professor interests 5", ButtonId(12)),
            create_button("Next", ButtonId(2)),
        ],
        // Third set of buttons
        vec![
            create_button("Back", ButtonId(0)),
            create_button("Allen, James F.", ButtonId(29)),
            create_button("Bai, Zhen", ButtonId(30)),
            create_button("Cetin, Mujdat", ButtonId(41)),
            create_button("Duan, Zhiyao", ButtonId(52)),
            create_button("Ferguson, George", ButtonId(63)),
            create_button("Ghoshal, Gourab", ButtonId(64)),
            create_button("Gildea, Daniel", ButtonId(75)),
            create_button("Haefner, Ralf", ButtonId(86)),
            create_button("He, Hangfeng", ButtonId(97)),
            create_button("Heyworth, Gregory", ButtonId(108)),
            create_button("Hoque, Ehsan", ButtonId(109)),
            create_button("Howard, Thomas M.", ButtonId(120)),
            create_button("Iordan, Coraline Rinn", ButtonId(121)),
            create_button("Jacobs, Robert A.", ButtonId(122)),
            create_button("Kahng, Anson", ButtonId(123)),
            create_button("Kanan, Christopher", ButtonId(134)),
            create_button("Kang, Jian", ButtonId(145)),
            create_button("Kautz, Henry", ButtonId(156)),
            create_button("Liang, Jiaming", ButtonId(167)),
            create_button("Luo, Jiebo", ButtonId(178)),
            create_button("Purtee, Adam", ButtonId(189)),
            create_button("Schubert, Lenhart K.", ButtonId(190)),
            create_button("Stefankovic, Daniel", ButtonId(191)),
            create_button("White, Aaron", ButtonId(202)),
            create_button("Xu, Chenliang", ButtonId(213)),
            create_button("Next", ButtonId(3)),
        ],
        // Fourth set of buttons
        vec![
            create_button("Back", ButtonId(0)),
            create_button("Bai, Zhen", ButtonId(30)),
            create_button("Hoque, Ehsan", ButtonId(109)),
            create_button("Yan, Yukang", ButtonId(3)),
            create_button("Zhu, Yuhao", ButtonId(294)),
            create_button("Next", ButtonId(4)),
        ],
        // Fifth set of buttons
        vec![
            create_button("Back", ButtonId(0)),
            create_button("Criswell, John", ButtonId(224)),
            create_button("Ding, Chen", ButtonId(235)),
            create_button("Dwarkadas, Sandhya", ButtonId(246)),
            create_button("Geng, Tong (Tony)", ButtonId(247)),
            create_button("Guo, Yanan", ButtonId(258)),
            create_button("Heinzelman, Wendi", ButtonId(259)),
            create_button("Huang, Michael", ButtonId(270)),
            create_button("Nargesian, Fatemeh", ButtonId(281)),
            create_button("Pai, Sreepathi", ButtonId(292)),
            create_button("Scott, Michael L.", ButtonId(303)),
            create_button("Zhu, Yuhao", ButtonId(304)),
            create_button("Next", ButtonId(5)),
        ],
        vec![
            create_button("Back", ButtonId(0)),
            create_button("Hemaspaandra, Lane A.", ButtonId(315)),
            create_button("Hosseini, Kaave", ButtonId(316)),
            create_button("Kahng, Anson", ButtonId(123)),
            create_button("Read-McFarland, Andrew", ButtonId(327)),
            create_button("Seiferas, Joel I.", ButtonId(328)),
            create_button("Stefankovic, Daniel", ButtonId(191)),
            create_button("Venkitasubramaniam, Muthuramakrishnan", ButtonId(329)),
            create_button("Zhupa, Eustrat", ButtonId(340)),
            create_button("Next", ButtonId(1)),
        ],
        //index 6
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Algorithms", ButtonId(13)),
            create_button("Artificial Intelligence", ButtonId(14)),
            create_button("Augmented/Virtual Reality", ButtonId(15)),
            create_button("Computer Architecture", ButtonId(16)),
            create_button("Complexity Theory", ButtonId(17)),
            create_button("Common-sense reasoning/planning", ButtonId(18)),
            create_button("Computer Vision", ButtonId(19)),
            create_button("Data management/science/mining", ButtonId(20)),
            create_button("Next", ButtonId(7)),
        ],        
        //index 7
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Theoretical Computer Science", ButtonId(21)),
            create_button("Fourier transform and Discrete Fourier analysis", ButtonId(22)),          
            create_button("Natural language understanding/processing/semantics", ButtonId(23)),
            create_button("Memory management/sharing/systems", ButtonId(24)),
            create_button("Machine learning", ButtonId(25)),
            create_button("Knowledge representation", ButtonId(26)),
            create_button("Performance evaluation/modeling", ButtonId(27)),
            create_button("Reasoning", ButtonId(28)),
            create_button("Next", ButtonId(8)),
        ],
        //index 8
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Additive Combinatorics", ButtonId(316)), // Hosseini Kaave
            create_button("Assistive Technology", ButtonId(30)), // Bai Zhen
            create_button("Automata", ButtonId(328)), // Seiferas Joel
            create_button("Audio Signal Processing", ButtonId(52)), // Duan Zhiyao
            create_button("AI accelerators", ButtonId(247)), // Geng Tong (Tony)
            create_button("Agent communication languages/architectures", ButtonId(63)), // Ferguson George
            create_button("Attention", ButtonId(121)), // Iordan Coraline
            create_button("Applied Machine Learning", ButtonId(134)), // Kanan Christopher
            create_button("Brain-computer/machine interfaces", ButtonId(41)), // Cetin Mujdat
            create_button("Biomedical sensing/imaging/analysis", ButtonId(41)), // Cetin Mujdat
            create_button("Biomedical Informatics", ButtonId(178)), // Luo Jiebo
            create_button("Computational Social Choice", ButtonId(123)), // Kahng Anson
            create_button("Compilers", ButtonId(292)), // Pai Sreepathi
            create_button("Cloud computing", ButtonId(259)), // Heinzelman Wendi
            create_button("Computer Imaging and Graphics", ButtonId(304)), // Zhu Yuhao
            create_button("Computational Art", ButtonId(304)), // Zhu Yuhao
            create_button("Computer-Supported Collaborative Work", ButtonId(30)), // Bai Zhen
            create_button("Computer Audition", ButtonId(52)), // Duan Zhiyao
            create_button("Computer Security", ButtonId(224)), // Criswell John
            create_button("Cognitive/Perceptual Learning", ButtonId(122)), // Jacobs Robert
            create_button("Next", ButtonId(9)),
        ],
        //index 9
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Cognitive Science", ButtonId(123)), // Kahng Anson
            create_button("Combinatorics", ButtonId(191)), // Stefankovic Daniel
            create_button("Computational Social Science", ButtonId(145)), // Kang Jian
            create_button("Computational Linguistics", ButtonId(202)), // White Aaron
            create_button("Cryptography", ButtonId(329)), // Venkitasubramaniam Muthuramakrishnan
            create_button("Democracy in CS", ButtonId(123)), // Kahng Anson
            create_button("Data discovery/integration", ButtonId(281)), // Nargesian Fatemeh
            create_button("Depth perception/processing of binocular disparity", ButtonId(86)), // Haefner Ralf
            create_button("Data science", ButtonId(156)), // Kautz Henry
            create_button("Energy-efficient computer microarchitecture", ButtonId(270)), // Huang Michael
            create_button("Embodied Conversational Agent", ButtonId(30)), // Bai Zhen
            create_button("Transactional memory", ButtonId(303)), // Scott Michael
            create_button("Tangible User Interfaced", ButtonId(30)), // Bai Zhen
            create_button("Technology-Enhanced Learning", ButtonId(30)), // Bai Zhen
            create_button("Theoretical condensed-matter physics", ButtonId(64)), // Ghoshal Gourab
            create_button("Trustworthy AI", ButtonId(145)), // Kang Jian
            create_button("User interfaces", ButtonId(63)), // Ferguson George
            create_button("Ubiquitous/Mobile Computing", ButtonId(178)), // Luo Jiebo
            create_button("Graph Theory", ButtonId(191)), // Stefankovic Daniel
            create_button("Next", ButtonId(10)),
        ],
        //index 10
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Game Theory", ButtonId(191)), // Stefankovic Daniel
            create_button("GPU algorithms", ButtonId(292)), // Pai Sreepathi
            create_button("GPU systems/architectures", ButtonId(258)), // Yanan Guo
            create_button("Interactive machine learning", ButtonId(109)), // Hoque Ehsan
            create_button("Non-von Neuman computing", ButtonId(270)), // Huang Michael
            create_button("Neural population encoding/decoding", ButtonId(86)), // Haefner Ralf
            create_button("Networks", ButtonId(246)), // Dwarkadas Sandhya
            create_button("Neuroplasticity", ButtonId(121)), // Iordan Coraline
            create_button("Neural networks", ButtonId(121)), // Iordan Coraline
            create_button("Neural Computation", ButtonId(122)), // Jacobs Robert
            create_button("Neumerical programming", ButtonId(189)), // Purtee Adam
            create_button("Marov chains/counting", ButtonId(191)), // Stefankovic Daniel
            create_button("Mobile ad hoc networks", ButtonId(259)), // Heinzelman Wendi
            create_button("Multimedia communication", ButtonId(259)), // Heinzelman Wendi
            create_button("Music Information Retrieval", ButtonId(52)), // Duan Zhiyao
            create_button("Machine translation", ButtonId(75)), // Gildea Daniel
            create_button("Motion Planning", ButtonId(120)), // Howard Thomas
            create_button("Multimodal Modeling", ButtonId(213)), // Xu Chenliang
            create_button("Learning theory", ButtonId(191)), // Stefankovic Daniel
            create_button("Locality theory and optimizations", ButtonId(235)), // Ding Chen
            create_button("Wireless sensor networks", ButtonId(259)), // Heinzelman Wendi
            create_button("Parallel and Distributed Systems", ButtonId(303)), // Scott Michael
            create_button("Next", ButtonId(11)),
        ],
        //index 11
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Phylogeny", ButtonId(191)), // Stefankovic Daniel
            create_button("Programming languages", ButtonId(303)), // Scott Michael
            create_button("Persistent memory", ButtonId(303)), // Scott Michael
            create_button("Perceptual decision-making", ButtonId(86)), // Haefner Ralf
            create_button("Probabilistic inference", ButtonId(86)), // Haefner Ralf
            create_button("Pervasive computing", ButtonId(156)), // Kautz Henry
            create_button("Pseudorandomness", ButtonId(316)), // Hosseini Kaave
            create_button("High-performance Computing", ButtonId(270)), // Huang Michael
            create_button("Energy-efficient computing microarchitecture", ButtonId(270)), // Huang Michael
            create_button("Heterogeneous networking", ButtonId(259)), // Heinzelman Wendi
            create_button("Heterogeneous Architectures", ButtonId(292)), // Pai Sreepathi
            create_button("Human Perception and Cognition", ButtonId(304)), // Zhu Yuhao
            create_button("Health and wellbeing", ButtonId(109)), // Hoque Ehsan
            create_button("Human Behavior Modeling", ButtonId(315)), // Yan Yukang
            create_button("Human-Robot Interaction", ButtonId(120)), // Howard Thomas
            create_button("Synchronization", ButtonId(303)), // Scott Michael
            create_button("Speech recognition", ButtonId(63)), // Ferguson George
            create_button("Semantic web", ButtonId(63)), // Ferguson George
            create_button("Next", ButtonId(12)),
        ],
        //index 12
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Semantic parsing", ButtonId(75)), // Gildea Daniel
            create_button("Secure Virtual Architecture", ButtonId(224)), // Criswell John
            create_button("Sampling", ButtonId(167)), // Liang Jiaming
            create_button("Social Media", ButtonId(178)), // Luo Jiebo
            create_button("Schema-based learning/behavior", ButtonId(190)), // Schubert Lenhart
            create_button("Self-motivated agents", ButtonId(190)), // Schubert Lenhart
            create_button("Routing", ButtonId(191)), // Stefankovic Daniel
            create_button("Real-time fMRI", ButtonId(121)), // Iordan Coraline
            create_button("Robotics", ButtonId(120)), // Howard Thomas
            create_button("Ising machines", ButtonId(270)), // Huang Michael
            create_button("Intelligen agents", ButtonId(63)), // Ferguson George
            create_button("Visual Semantic Cognition", ButtonId(121)), // Iordan Coraline
            create_button("Video Analysis", ButtonId(213)), // Xu Chenliang
            create_button("Optimization", ButtonId(167)), // Liang Jiaming
            create_button("Next", ButtonId(6)),
        ],
        //index 13
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(6)),
            create_button("Algorithms", ButtonId(13)),
            create_button("Gildea, Daniel", ButtonId(75)),
            create_button("Seiferas Joel", ButtonId(328)),
        ],
        //index 14
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(6)),
            create_button("Artificial Intelligence", ButtonId(14)),
            create_button("Hoque Ehsan", ButtonId(109)),
            create_button("Kahng Anson", ButtonId(123)),
            create_button("Ferguson George", ButtonId(63)),
            create_button("Howard Thomas", ButtonId(120)),
            create_button("Kanan Christopher", ButtonId(134)),
            create_button("Kautz Henry", ButtonId(156)),
        ],
        //index 15
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(6)),
            create_button("Augmented/Virtual Reality", ButtonId(15)),
            create_button("Bai Zhen", ButtonId(30)),
            create_button("Zhu Yuhao", ButtonId(304)),
            create_button("Yan Yukang", ButtonId(315)),
        ],
        //index 16
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(6)),
            create_button("Computer Architecture", ButtonId(16)),
            create_button("Dwarkadas Sandhya", ButtonId(246)),
            create_button("Geng Tong (Tony)", ButtonId(247)),
            create_button("Zhu Yuhao", ButtonId(304)),
        ],
        //index 17
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(6)),
            create_button("Complexity Theory", ButtonId(17)),
            create_button("Hemaspaandra Lane", ButtonId(315)),
            create_button("Venkitasubramaniam Muthuramakrishnan", ButtonId(329)),
        ],
        //index 18
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(6)),
            create_button("Common-sense/temporal reasoning/planning", ButtonId(18)),
            create_button("Allen James", ButtonId(29)),
            create_button("Ferguson George", ButtonId(63)),
            create_button("He Hangfeng", ButtonId(97)),
            create_button("Purtee Adam", ButtonId(189)),
        ],
        //index 19
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(6)),
            create_button("Computer Vision", ButtonId(19)),
            create_button("Xu Chenliang", ButtonId(213)),
            create_button("Cetin Mujdat", ButtonId(41)),
            create_button("Kanan Christopher", ButtonId(134)),
            create_button("Luo Jiebo", ButtonId(178)),
            create_button("Kanan Christopher", ButtonId(134)),
        ],
        //index 20
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(6)),
            create_button("Data management/science/mining", ButtonId(20)),
            create_button("Nargesian Fatemeh", ButtonId(281)),
            create_button("Ghoshal Gourab", ButtonId(64)),
            create_button("Iordan Coraline", ButtonId(121)),
            create_button("Kautz Henry", ButtonId(156)),
            create_button("Luo Jiebo", ButtonId(178)),
            create_button("Kang Jian", ButtonId(145)),
        ],
        //index 21
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(7)),
            create_button("Theoretical Computer Science", ButtonId(21)),
            create_button("Hosseini Kaave", ButtonId(316)),
            create_button("Kahng Anson", ButtonId(123)),
            create_button("Seiferas Joel", ButtonId(328)),
            create_button("Stefankovic Daniel", ButtonId(191)),
        ],
        //index 22
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(7)),
            create_button("Fourier transform and Discrete Fourier analysis", ButtonId(22)),
            create_button("Stefankovic Daniel", ButtonId(191)),
            create_button("Hosseini Kaave", ButtonId(316)),
        ],
        //index 23
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(7)),
            create_button("Natural language understanding/processing/semantics", ButtonId(23)),
            create_button("Allen James", ButtonId(29)),
            create_button("Ferguson George", ButtonId(63)),
            create_button("Gildea Daniel", ButtonId(75)),
            create_button("He Hangfeng", ButtonId(97)),
            create_button("Purtee Adam", ButtonId(189)),
            create_button("White Aaron", ButtonId(202)),
        ],
        //index 24
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(7)),
            create_button("Memory management/sharing/systems", ButtonId(24)),
            create_button("Ding Chen", ButtonId(235)),
            create_button("Dwarkadas Sandhya", ButtonId(246)),
            create_button("Yanan Guo", ButtonId(258)),
            create_button("Scott Michael", ButtonId(303)),
        ],
        //index 25
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(7)),
            create_button("Machine learning", ButtonId(25)),
            create_button("Geng Tong (Tony)", ButtonId(247)),
            create_button("Bai Zhen", ButtonId(30)),
            create_button("Hoque Ehsan", ButtonId(109)),
            create_button("Cetin Mujdat", ButtonId(41)),
            create_button("Duan Zhiyao", ButtonId(52)),
            create_button("He Hangfeng", ButtonId(97)),
            create_button("Iordan Coraline", ButtonId(121)),
            create_button("Kanan Christopher", ButtonId(134)),
            create_button("Kang Jian", ButtonId(145)),
            create_button("Luo Jiebo", ButtonId(178)),
            create_button("Purtee Adam", ButtonId(189)),
            create_button("Xu Chenliang", ButtonId(213)),
        ],
        //index 26
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(7)),
            create_button("Knowledge representation", ButtonId(26)),
            create_button("Allen James", ButtonId(29)),
            create_button("Ferguson George", ButtonId(63)),
            create_button("Gildea Daniel", ButtonId(75)),
            create_button("Schubert Lenhart", ButtonId(190)),
            create_button("Purtee Adam", ButtonId(189)),
        ],
        //index 27
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(7)),
            create_button("Performance evaluation/modeling", ButtonId(27)),
            create_button("Dwarkadas Sandhya", ButtonId(246)),
            create_button("Pai Sreepathi", ButtonId(292)),
        ],
        //index 28
        vec![
            create_button("Home", ButtonId(0)),
            create_button("Back", ButtonId(7)),
            create_button("Reasoning", ButtonId(28)),
            create_button("Allen James", ButtonId(29)),
            create_button("Ferguson George", ButtonId(63)),
            create_button("He Hangfeng", ButtonId(97)),
            create_button("Purtee Adam", ButtonId(189)),
        ]
    ]);

        // Vector of professor names
    let professors = vec![
        ("James", "Allen"),
        ("Zhen", "Bai"),
        ("Mujdat", "Cetin"),
        ("Zhiyao", "Duan"),
        ("George", "Ferguson"),
        ("Gourab", "Ghoshal"),
        ("Daniel", "Gildea"),
        ("Ralf", "Haefner"),
        ("Hangfeng", "He"),
        ("Gregory", "Heyworth"),
        ("Ehsan", "Hoque"),
        ("Thomas M.", "Howard"),
        ("Coraline Rinn", "Iordan"),
        ("Robert A.", "Jacobs"),
        ("Anson", "Kahng"),
        ("Christopher", "Kanan"),
        ("Jian", "Kang"),
        ("Henry", "Kautz"),
        ("Jiaming", "Liang"),
        ("Jiebo", "Luo"),
        ("Adam", "Purtee"),
        ("Lenhart K.", "Schubert"),
        ("Daniel", "Stefankovic"),
        ("Aaron", "White"),
        ("Chenliang", "Xu"),
        ("John", "Criswell"),
        ("Chen", "Ding"),
        ("Sandhya", "Dwarkadas"),
        ("Tong", "Geng"),
        ("Yanan", "Guo"),
        ("Wendi", "Heinzelman"),
        ("Michael", "Huang"),
        ("Fatemeh", "Nargesian"),
        ("Sreepathi", "Pai"),
        ("Michael L.", "Scott"),
        ("Yuhao", "Zhu"),
        ("Lane A.", "Hemaspaandra"),
        ("Kaave", "Hosseini"),
        ("Andrew", "Read-McFarland"),
        ("Joel I.", "Seiferas"),
        ("Muthuramakrishnan", "Venkitasubramaniam"),
        ("Eustrat", "Zhupa"),
    ];

    // Starting index for button IDs

    // Loop through each professor and add their button set to `button_set`
    let file_path = env::var("CS_JSON_PATH").unwrap_or("src/cs_1723089902.json".to_string());
    for (first_name, last_name) in professors {
        let index = button_set.0.len();
        println!("Starting index: {} {} {}", index, first_name, last_name);
        let professor_set = create_professor_button_set(
            search_by_name(&file_path, first_name, last_name),
            index,
            &first_name,
            &last_name
        );
        for set in professor_set {
            button_set.0.push(set);
        }
    }

    App::new()
        .insert_resource(button_set)
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &ButtonId,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    button_sets: Res<ButtonSets>,
    asset_server: Res<AssetServer>,
    button_query: Query<Entity, With<Button>>,
) {
    for (interaction, mut color, mut border_color, children, button_id) in &mut interaction_query {
        let _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                println!("Button with ID {} was clicked!", button_id.0);

                for entity in button_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                
                if let Some(_button_set) = button_sets.0.get(button_id.0) {
                    spawn_buttons(&mut commands, &asset_server, button_sets.0[button_id.0].clone());
                }
        }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, button_sets: Res<ButtonSets>,) {
    commands.spawn(Camera2dBundle::default());
    spawn_buttons(&mut commands, &asset_server, button_sets.0[0].clone());
}

fn spawn_buttons(commands: &mut Commands, asset_server: &Res<AssetServer>, button_data: Vec<ButtonData>) {

    let total_buttons = button_data.len();
    let max_columns = 6; // Maximum number of columns (this can be adjusted)
    let max_rows = 6; // Maximum number of rows (this can be adjusted)

    let columns = if total_buttons < max_columns {
        total_buttons
    } else {
        max_columns
    };
    let rows = (total_buttons + columns - 1) / columns;
    let rows = if rows > max_rows { max_rows } else { rows };

    // let rows = 4; // Set the number of columns (you can change this)
    // let columns = (button_data.len() + rows - 1) / rows; // Calculate the number of rows

    // Spawn a parent node that will hold the grid structure
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column, // Stack rows vertically
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Iterate over the number of rows
            for row in 0..rows {
                // Spawn a row container that holds buttons for this row
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row, // Arrange buttons horizontally within each row
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row_parent| {
                    // Iterate over buttons in the current row
                    for col in 0..columns {
                        let index = row * columns + col; // Calculate the index in the flattened button data list

                        if index < button_data.len() {
                            let data = &button_data[index];

                            row_parent
                                .spawn(data.button_bundle.clone())
                                .insert(data.button_id.clone())
                                .with_children(|button_parent| {
                                    button_parent.spawn(TextBundle::from_section(
                                        &data.button_text,
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 16.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });
                        }
                    }
                });
            }
        });
    }