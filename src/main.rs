use std::fs::File;
use std::io::{BufRead};
use std::io;

mod graph;
use graph::Graph;

mod route;
use route::Route;

pub fn read_file(path: &str) -> Vec<Vec<i64>> {
    let file = File::open(path).expect("could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    let mut result: Vec<Vec<i64>> = Vec::new();
    for (idx, line) in buf_reader.enumerate() {
        match line {
            Ok(content) => {
                let rows: Vec<i64>;
                if idx == 0 {
                    let content_without_bom = content.trim_start_matches('\u{feff}');
                    rows = content_without_bom.split(",").map(|s| s.trim().parse().unwrap()).collect();
                } else {
                    rows = content.split(",").map(|s| s.trim().parse().unwrap()).collect();
                }
                result.push(rows);
            }
            Err(_) => println!("Error reading line"),
        }
    }

    return result;
}

fn select_three(mut paths:Route){
    let mut start_input = String::new();
        println!("Enter Start Node: ");

        io::stdin().read_line(&mut start_input).expect("Fail to read line");

        let parse_start: Result<i64, _> = start_input.trim().parse();
        match parse_start {
            Ok(number) => {
                let mut end_input = String::new();
                if number == -1 {
                    println!("Program End");
                    return;
                }
                println!("Enter End Node: ");
                io::stdin().read_line(&mut end_input).expect("Fail to read line");

                let parse_end: Result<i64, _> = end_input.trim().parse();
                match parse_end {
                    Ok(number2) => {
                        println!("Start {} Node to End {} Node Shortest Path: ", number, number2);
                        println!("{:?}",paths.get_route(number,number2));
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        println!("------------------------\n\n");
    
}

#[test]
fn test_graph() {
    let nodes:Vec<i64> = vec![0,1,2,3,4];
    let mut graph = Graph::new();
    graph.add_edge(nodes[0],nodes[1]);
    graph.add_edge(nodes[0],nodes[2]);
    graph.add_edge(nodes[1],nodes[2]);
    graph.add_edge(nodes[1],nodes[3]);
    graph.add_edge(nodes[2],nodes[3]);
    graph.add_edge(nodes[3],nodes[2]);
    assert_eq!(graph.nodes,[0, 1, 2, 3]);
    assert_eq!(graph.edges,vec![vec![1, 2], vec![2, 3], vec![3], vec![2]]);
}

#[test]
fn test_route() {
    let nodes:Vec<i64> = vec![0,1,2,3,4];
    let mut graph = Graph::new();
    graph.add_edge(nodes[0],nodes[1]);
    graph.add_edge(nodes[0],nodes[2]);
    graph.add_edge(nodes[1],nodes[2]);
    graph.add_edge(nodes[1],nodes[3]);
    graph.add_edge(nodes[2],nodes[3]);
    graph.add_edge(nodes[3],nodes[2]);
    let mut paths = Route::new(graph);
    assert_eq!(paths.get_route(0,0),[0]);
    assert_eq!(paths.get_route(0,1),[0, 1]);
    assert_eq!(paths.get_route(0,2),[0, 2]);
    assert_eq!(paths.get_route(0,3),[0, 1, 3]);
    assert_eq!(paths.get_route(0,4),[]);
    assert_eq!(paths.get_route(1,2),[1, 2]);
    assert_eq!(paths.get_route(1,3),[1, 3]);
    assert_eq!(paths.get_route(3,1),[]);
    assert_eq!(paths.get_route(3,2),[3, 2]);
}

fn select_second(paths:Route){
    let mut start_input = String::new();
        println!("Enter Node: ");

        io::stdin().read_line(&mut start_input).expect("Fail to read line");

        let parse_start: Result<i64, _> = start_input.trim().parse();
        match parse_start {
            Ok(number) => {
                if paths.graph.find_node_index(number) == -1 {
                    println!("This node does not exist.");
                    return;
                }
                paths.graph.print_edge(number);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
}

fn main() {
    
    let result = read_file("data/euroroad.csv");
    let mut graph = Graph::new();

    for items in result{
        graph.add_edge(items[0],items[1]);
    }
    let paths = Route::new(graph);

    loop{
        println!("1. Graph output\n2. Edge search\n3. Find route\n4. End\nselect");
        let mut select = String::new();
        io::stdin().read_line(&mut select).expect("Fail to read line");
        let select_p: Result<i64, _> = select.trim().parse();
        match select_p {
            Ok(number) => {
                if number == 1{
                    paths.graph.print_graph();
                }else if number == 2{
                    select_second(paths.clone());
                }else if number == 3{
                    select_three(paths.clone());
                }else if number==4{
                    println!("—————Program End—————");
                    return;
                }
            }Err(e) => {
                println!("Error: {}", e);
            }
        }

    }
    
}
