use std::cell::RefCell;
use std::rc::Rc;
use std::panic::panic_any;
use std::slice::RChunks;



#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}
type Tree = Rc<RefCell<TreeNode<u32>>>;
type RedBlackTree= Option<Tree>;
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}
struct RBTree{
    root:RedBlackTree,
}
/************TreeNode***************/
impl<T:Ord> TreeNode<T> {
    fn new(val: T) -> Self {
        TreeNode {
            color: NodeColor::Black,
            key: val,
            parent: None,
            left: None,
            right: None,
        }
    }
}
/***********RbTree****************/
impl RBTree{
    fn new() -> Self{
        RBTree{
            root:None,
        }
    }
    fn is_left_side(option_node:&Tree) -> bool{
        let node = option_node.borrow();
        let parent_option = &node.parent.as_ref().unwrap();
        let parent_node = parent_option.borrow();
        match parent_node.left.as_ref(){
            Some(x) => x.borrow().key==node.key,
            None => false,
        }
    }
    fn get_sibiling_node(node_p:&Tree) ->RedBlackTree {
        let node = node_p.borrow();
        if node.parent.is_some(){
            let parent = &node.parent.as_ref().unwrap();
            let parent_node = parent.borrow();
            if RBTree::is_left_side(node_p){
                return parent_node.right.clone();
            }
            return parent_node.left.clone();
        }
        return None;
    }
    fn get_parent(node_p:&Tree) ->RedBlackTree {
        let node = node_p.borrow();
        if node.parent.is_some(){
            return Some(Rc::clone(node.parent.as_ref().unwrap()));
        }
        return None;
    }
    fn get_color(node:&Tree) -> NodeColor{
        let node = node.borrow();
        if node.color == NodeColor::Red{
            return NodeColor::Red;
        }
        NodeColor::Black
    }
    fn has_red_child(node_p:&Tree) -> bool{
        let node = node_p.borrow();
        if node.left.is_some() && RBTree::get_color(node.left.as_ref().unwrap()) == NodeColor::Red{
            return true;
        }
        if node.right.is_some() && RBTree::get_color(node.right.as_ref().unwrap()) == NodeColor::Red{
            return true;
        }
        false
    }
    fn get_key(node_p:&Tree) -> u32{
        let node = node_p.borrow();
        node.key
    }
    fn reset_color(node:&mut &Tree,new_color:NodeColor){
        let mut node = node.borrow_mut();
        node.color = new_color;
    }
    fn private_get_number_leaves(node_op:&RedBlackTree,mut count:u32) ->u32{
        let node = node_op.as_ref().unwrap().borrow_mut();
        if node.left.is_some(){
            count = RBTree::private_get_number_leaves(&node.left,count);
        }
        if node.right.is_some(){
            count = RBTree::private_get_number_leaves(&node.right,count);
        }
        if node.left.is_none()&&node.right.is_none(){
            count=count+1;
        }
        count
    }
    pub fn is_empty(&self) -> bool{
        self.root.is_none()
    }
    pub fn get_number_leaves(&self) -> u32{
        let mut count:u32 = 0;
        if self.is_empty(){
            return count;
        }
        else{
            count = RBTree::private_get_number_leaves(&self.root,count);
        }
        count
    }
    fn private_get_height(node_op:&RedBlackTree) -> u32{
        if node_op.is_none(){
            return 0u32;
        }
        let node = node_op.as_ref().unwrap().borrow_mut();
        let left_height:u32 = RBTree::private_get_height(&node.left);
        let right_height:u32 = RBTree::private_get_height(&node.right);
        if left_height>right_height {
            return left_height + 1;
        }
        right_height + 1
    }
    pub fn get_height(&self) ->u32{
        if self.is_empty(){
            return 0u32;
        }
        RBTree::private_get_height(&self.root)
    }
    fn left_rotation(&mut self,node:&Tree){
        {
            let parent_option = &node.borrow().parent;
            let right_option = &node.borrow().right;
            if node.borrow().parent.is_none(){
                self.root = right_option.clone();
            }
            if let Some(parent_node) = parent_option{
                if RBTree::is_left_side(node){
                    parent_node.borrow_mut().left = right_option.clone();
                }
                else{
                    parent_node.borrow_mut().right = right_option.clone();
                }
            }
            right_option.as_ref().unwrap().borrow_mut().parent = parent_option.clone();
        }
        let right_node = node.borrow().right.as_ref().unwrap().clone();
        node.borrow_mut().parent = Some(Rc::clone(&right_node));
        if right_node.borrow().left.is_some(){
            node.borrow_mut().right = Some(right_node.borrow().left.as_ref().unwrap().clone());
            right_node.borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::clone(node));
        }
        else{
            node.borrow_mut().right=None;
        }
        right_node.borrow_mut().left = Some(node.clone());
    }
    fn right_rotation(&mut self,node:&Tree){
        {
            let parent_option = &node.borrow().parent;
            let left_option = &node.borrow().left;
            if node.borrow().parent.is_none(){
                self.root = left_option.clone();
            }
            if let Some(parent_node) = parent_option{
                if RBTree::is_left_side(node){
                    parent_node.borrow_mut().left = left_option.clone();
                }
                else{
                    parent_node.borrow_mut().right = left_option.clone();
                }
            }
            left_option.as_ref().unwrap().borrow_mut().parent = parent_option.clone();
        }
        let left_node = node.borrow().left.as_ref().unwrap().clone();
        node.borrow_mut().parent=Some(Rc::clone(&left_node));
        if left_node.borrow().right.is_some(){
            node.borrow_mut().left = Some(left_node.borrow().right.as_ref().unwrap().clone());
            left_node.borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::clone(node));
        }
        else{
            node.borrow_mut().left = None;
        }
        left_node.borrow_mut().right = Some(node.clone());
    }
    fn find_right_child(node: RedBlackTree) ->RedBlackTree {
        if !node.as_ref().unwrap().borrow().right.is_none(){
            return Self::find_right_child(node.as_ref().unwrap().borrow().right.clone());
        }
        return node;
    }
    fn find_replace_node(node:&Tree) -> RedBlackTree{
        let node = node.borrow();
        if node.left.is_some() && node.right.is_some(){
            return Self::find_right_child(node.left.clone());
        }
        else if node.left.is_some(){
            return node.left.clone();
        }
        else if node.right.is_some(){
            return node.right.clone();
        }
        else{
            return None;
        }
    }
    fn private_search(&mut self,val:u32) ->(bool,RedBlackTree) {
        if self.root.is_none(){
            return (false,None);
        }
        let mut option_parent = None;
        let mut option_children = Some(self.root.as_ref().unwrap().clone());
        while !option_children.is_none(){
            option_parent = option_children;
            let parent_node = option_parent.as_ref().unwrap();
            let parent_node_val = parent_node.borrow().key;
            if parent_node_val<val{
                option_children = match parent_node.borrow().right{
                    Some(ref option_node) => (Some(option_node.clone())),
                    None => None,
                };
            }else if parent_node_val>val{
                option_children = match parent_node.borrow().left {
                    Some(ref option_node) => (Some(option_node.clone())),
                    None => None,
                };
            }else{
                return (true,option_parent);
            }
        }
        return (false,option_parent);
    }
    fn private_delete_node(&mut self,node_to_delete:&mut &Tree) ->Result<(),String> {
        let replace_node = RBTree::find_replace_node(node_to_delete);
        let replace_delete_black:bool = (replace_node.is_none()||RBTree::get_color(replace_node.as_ref().unwrap())==NodeColor::Black)&&RBTree::get_color(node_to_delete)==NodeColor::Black;
        let parent = if node_to_delete.borrow().parent.is_some(){
            Some(Rc::clone(node_to_delete.borrow().parent.as_ref().unwrap()))
        }
        else{
            None
        };
        //node_to_delete has 0 child
        if replace_node.is_none(){
            //replace_node is null
            if node_to_delete.borrow().parent.is_none(){
                //node_to_delete is root
                self.root = None;
            }
            else{
                if replace_delete_black{
                    //replace_node and node_to_delete are both black.node_to_delete is leaf
                    self.adjust_double_black(node_to_delete);
                }
                if RBTree::is_left_side(node_to_delete){
                    parent.as_ref().unwrap().borrow_mut().left = None;
                }else{
                    parent.as_ref().unwrap().borrow_mut().right = None;
                }
            }
            return Ok(());
        }
        //node_to_delete has 1 child
        else if node_to_delete.borrow().left.is_none() || node_to_delete.borrow().right.is_none(){
            if node_to_delete.borrow().parent.is_none(){
                //node_to_delete is root
                let replace_key = RBTree::get_key(replace_node.as_ref().unwrap());
                let mut root = self.root.as_ref().unwrap().borrow_mut();
                root.key = replace_key;
                root.left = None;
                root.right = None;
            }else {
                //reset the child
                if RBTree::is_left_side(node_to_delete){
                    parent.as_ref().unwrap().borrow_mut().left = replace_node.clone();
                }else {
                    parent.as_ref().unwrap().borrow_mut().right = replace_node.clone();
                }
                //reset the parent
                replace_node.as_ref().unwrap().borrow_mut().parent = parent.clone();
                if replace_delete_black{
                    self.adjust_double_black(replace_node.as_ref().unwrap())
                }else {
                    RBTree::reset_color(&mut replace_node.as_ref().unwrap(),NodeColor::Black);
                }
            }
            return Ok(());
        }else {
            // node_to_delete has 2 children, change the value of node_to_delete and use recursion to delete replace_node
            let replace_key = RBTree::get_key(replace_node.as_ref().unwrap());
            node_to_delete.borrow_mut().key = replace_key;
            self.private_delete_node(&mut replace_node.as_ref().unwrap())?;
            return Ok(());
        }
    }
    fn adjust_double_black(&mut self,node:&Tree){
        if node.borrow().parent.is_none(){
            return;
        }
        let node_s = RBTree::get_sibiling_node(node);
        let node_p = RBTree::get_parent(node);

        if node_s.is_none(){
            self.adjust_double_black(&node_p.unwrap());
        }else {
            if RBTree::get_color(node_s.as_ref().unwrap()) == NodeColor::Black{
                //node_s is black
                if RBTree::has_red_child(node_s.as_ref().unwrap()){
                    //the child of node_s at least is red
                    if node_s.as_ref().unwrap().borrow().left.is_some()
                        &&RBTree::get_color(node_s.as_ref().unwrap().borrow().left.as_ref().unwrap())==NodeColor::Red{
                        if RBTree::is_left_side(node_s.as_ref().unwrap()){
                            //node_s is left, and left child of node_s is red LL
                            let s_color = RBTree::get_color(node_s.as_ref().unwrap());
                            let p_color = RBTree::get_color(node_p.as_ref().unwrap());
                            //reset color of left_node_s to s_color, reset color of node_s to p_color
                            RBTree::reset_color(&mut node_s.as_ref().unwrap().borrow_mut().left.as_ref().unwrap(),s_color);
                            RBTree::reset_color(&mut node_s.as_ref().unwrap(),p_color);
                            //do right rotation to node_p
                            self.right_rotation(node_p.as_ref().unwrap());
                        }else {
                            //node_s is right, and left child of node_s is red RL
                            let p_color = RBTree::get_color(node_p.as_ref().unwrap());
                            //reset the color of left child of node_s to p_color
                            RBTree::reset_color(&mut node_s.as_ref().unwrap().borrow_mut().left.as_ref().unwrap(),p_color);
                            //do right_rotation to node_s, do left_rotation to node_p
                            self.right_rotation(node_s.as_ref().unwrap());
                            self.left_rotation(node_p.as_ref().unwrap());
                        }
                    }else{
                        if RBTree::is_left_side(node_s.as_ref().unwrap()){
                            //node_s is left, the right child of node_s is red LR
                            let p_color = RBTree::get_color(node_p.as_ref().unwrap());
                            //reset the color of right child to p_color
                            RBTree::reset_color(&mut node_s.as_ref().unwrap().borrow_mut().right.as_ref().unwrap(),p_color);
                            //do left_rotation to node_s, do right_rotation to node_p
                            self.left_rotation(node_s.as_ref().unwrap());
                            self.right_rotation(node_p.as_ref().unwrap());
                        }else {
                            //node_s is right, the right child of node_s is red RR
                            let s_color = RBTree::get_color(node_s.as_ref().unwrap());
                            let p_color = RBTree::get_color(node_p.as_ref().unwrap());
                            //reset the color of right child to s_color, reset the s_color to p_color
                            RBTree::reset_color(&mut node_s.as_ref().unwrap().borrow_mut().right.as_ref().unwrap(),s_color);
                            RBTree::reset_color(&mut node_s.as_ref().unwrap(),p_color);
                            //do left_rotation to node_p
                            self.left_rotation(node_p.as_ref().unwrap());
                        }
                    }
                    RBTree::reset_color(&mut node_p.as_ref().unwrap(),NodeColor::Black);
                }else {
                    //2 black children
                    RBTree::reset_color(&mut node_s.as_ref().unwrap(),NodeColor::Black);
                    if RBTree::get_color(node_p.as_ref().unwrap())== NodeColor::Black{
                        self.adjust_double_black(node_p.as_ref().unwrap());
                    }else {
                        RBTree::reset_color(&mut node_p.as_ref().unwrap(),NodeColor::Black);
                    }
                }

            }else{
                //node_s is red
                //reset the node_s  be black, reset the node_p be red,
                RBTree::reset_color(&mut node_p.as_ref().unwrap(),NodeColor::Red);
                RBTree::reset_color(&mut node_s.as_ref().unwrap(),NodeColor::Black);
                if RBTree::is_left_side(node_s.as_ref().unwrap()){
                    //s is left child,node_p do right rotation
                    self.right_rotation(node_p.as_ref().unwrap());
                }else{
                    // s is right child,node_p do left rotation
                    self.left_rotation(node_p.as_ref().unwrap());
                }
            }
        }
    }
    pub fn search_node(&mut self,val:u32) -> Result<(),String>{
        match self.private_search(val){
            (false,_) => Err(format!("The node with val is not found").to_string()),
            (true,_) => Ok(()),
        }
    }

    pub fn delete(&mut self,val:u32) -> Result<(), String>{
        if self.root.is_none() {
            return Err(format!("Tree is none").to_string());
        }
        let (is_found,option_node_to_delete) = self.private_search(val);
        if !is_found {
            return Err(format!("The node with val is not found").to_string());
        }
        let mut node_to_delete = option_node_to_delete.as_ref().unwrap();
        self.private_delete_node(&mut node_to_delete)
    }

}