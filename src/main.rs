mod structure;
mod tool;

use crate::structure::bst::BstNode;
use crate::structure::tree::Node;
use crate::structure::tree::NodeLink;
use crate::structure::bst::BstNodeLink;
use crate::tool::generate_dotfile;
use crate::tool::generate_dotfile_bst;

fn main() {
    //turn on to test the old code
    // test_binary_tree(); 
    test_binary_search_tree();
}

fn test_binary_search_tree() {
    let rootlink: BstNodeLink = BstNode::new_bst_nodelink(15);
    rootlink.borrow_mut().add_left_child(&rootlink, 6);
    rootlink.borrow_mut().add_right_child(&rootlink, 18);

    //add right subtree
    if let Some(right_tree_extract) = &rootlink.borrow().right {
        right_tree_extract.borrow_mut().add_left_child(right_tree_extract, 17);
        right_tree_extract.borrow_mut().add_right_child(right_tree_extract, 20);
    }

    //add left subtree
    if let Some(left_tree_extract) = &rootlink.borrow().left {
        left_tree_extract.borrow_mut().add_left_child(left_tree_extract, 3);
        left_tree_extract.borrow_mut().add_right_child(left_tree_extract, 7);

        //add left subtree terminal
        if let Some(terminal_left_tree_link) = &left_tree_extract.borrow().left {
            terminal_left_tree_link.borrow_mut().add_left_child(terminal_left_tree_link, 2);
            terminal_left_tree_link.borrow_mut().add_right_child(terminal_left_tree_link, 4);
        }

        //add 2nd level right subtree of node 7
        if let Some(second_right_subtree_link) = &left_tree_extract.borrow().right {
            second_right_subtree_link.borrow_mut().add_right_child(second_right_subtree_link, 13);
            if let Some(third_left_subtree_link) = &second_right_subtree_link.borrow().right {
                third_left_subtree_link.borrow_mut().add_left_child(third_left_subtree_link, 9);
            }
        }
    }

    //print the tree at this time
    let main_tree_path = "bst_graph.dot";
    generate_dotfile_bst(&rootlink, main_tree_path);

    //tree search test
    let search_keys = vec![15, 9, 22];

    for &key in &search_keys {
        print!("tree search result of key {} is ", key);
        if let Some(node_result) = rootlink.borrow().tree_search(&key) {
            println!("found -> {:?}", node_result.borrow().key);
        } else {
            println!("not found");
        }
    }

    //min test
    let min_node = rootlink.borrow().minimum();
    println!("minimum result {:?}", min_node.borrow().key);

    //max test
    let max_node = rootlink.borrow().maximum();
    println!("maximum result {:?}", max_node.borrow().key);

    //root node get test
    let root_node = BstNode::get_root(&max_node);
    println!("root node {:?}", root_node.borrow().key);

    //successor test
    let query_keys = vec![2, 20, 15, 13, 9, 7, 22];
    for &key in &query_keys {
        if let Some(node) = rootlink.borrow().tree_search(&key) {
            print!("successor of node ({}) is ", key);
            if let Some(successor) = BstNode::tree_successor_simpler(&node) {
                println!("{:?}", successor.borrow().key);
            } else {
                println!("not found");
            }
        } else {
            println!("node with key of {} does not exist, failed to get successor", key);
        }
    }

    // === INSERT TEST ===
    println!("\n--- INSERT TEST ---");
    let new_node = BstNode::new_bst_nodelink(2201);
    BstNode::tree_insert_recursive(&mut Some(rootlink.clone()), new_node.clone());

    let insert_path = "bst_after_insert.dot";
    generate_dotfile_bst(&rootlink, insert_path);
    println!("Inserted node 8, DOT file written to {}", insert_path);

    // === DELETE TEST ===
    println!("\n--- DELETE TEST ---");
    {
        let target_node_opt = rootlink.borrow().tree_search(&4);
        if let Some(target_node) = target_node_opt {
            BstNode::tree_delete_recursive(&mut Some(rootlink.clone()), &target_node);

            let delete_path = "bst_after_delete.dot";
            generate_dotfile_bst(&rootlink, delete_path);
            println!("Deleted node 4, DOT file written to {}", delete_path);
        } else {
            println!("Node 4 not found for deletion.");
        }
    }

}

#[allow(dead_code)]
fn test_binary_tree() {
    //create the nodelink of the root node
    let rootlink: NodeLink = Node::new_nodelink(5);

    //add a new left node value
    rootlink.borrow_mut().add_left_child(&rootlink, 3);
    //add a new right node value
    rootlink.borrow_mut().add_right_child(&rootlink, 7);

    //println!("{:?}", rootlink);

    //print the tree at this time
    generate_dotfile(&rootlink, "prime.dot");

    //add new child values to the left subtree
    if let Some(left_tree_extract) = &rootlink.borrow().left {
        left_tree_extract.borrow_mut().add_left_child(left_tree_extract, 2);
        left_tree_extract.borrow_mut().add_right_child(left_tree_extract, 4);
    }

    //add new child values to the right subtree
    if let Some(right_tree_extract) = &rootlink.borrow().right {
        right_tree_extract.borrow_mut().add_right_child(right_tree_extract, 10);
    }

    //print the tree again, now been added with more values
    generate_dotfile(&rootlink, "prime_t2.dot");

    //Call tree depth function at this time
    let recorded_depth = rootlink.borrow().tree_depth();
    println!("Current tree depth: {}", recorded_depth);

    //Call count_nodes function
    let total_nodes = rootlink.borrow().count_nodes();
    println!("Amount of nodes in current tree: {}", total_nodes);

    //Call count_nodes_by_nodelink function, supplied right subtree as parameter
    //TODO
    let subtree_count = Node::count_nodes_by_nodelink(&rootlink.borrow().right.clone().unwrap(), 0);
    println!("Amount of nodes in current subtree: {}", subtree_count);

    //Get the sibling of the leftsubtree from parent
    let left_subtree = rootlink.borrow().get_node_by_value(3);
    println!("left subtree seek by value {:?}", left_subtree);

    //get the left subtree by full properties
    let another_left_subtree = rootlink
        .borrow()
        .get_node_by_full_property(&left_subtree.as_ref().unwrap());
    println!("left subtree seek by full property {:?}", another_left_subtree);

    //Discard the right subtree from parent
    let rootlink2 = rootlink.borrow().get_nodelink_copy();

    let flag = rootlink2.borrow_mut().discard_node_by_value(3);
    println!("status of node deletion: {}", flag);

    //print the tree again
    generate_dotfile(&rootlink2, "prime_t3.dot");

    //Call tree depth function at this time
    //TODO
    let depth_now = rootlink2.borrow().tree_depth();
    println!("Depth after discard {}", depth_now);

    //Call count_nodes function
    let count_now = rootlink2.borrow().count_nodes();
    println!("Count nodes after discard {}", count_now);

    //print the tree again
    generate_dotfile(&rootlink, "prime_t4.dot");
}
