use core::cmp::{max, Ordering};
use core::mem::swap;

use DeleteValue::*;
use InnerResult::*;

pub type AvlTreeNode<T> = Option<Box<TreeNode<T>>>;

#[derive(Clone, Debug)]
pub struct TreeNode<T: PartialOrd> {
    val: T,
    height: i32,
    left: AvlTreeNode<T>,
    right: AvlTreeNode<T>,
}

enum InnerResult {
    Left,
    //在左子树完成插入
    Right,
    //在右子树完成插入
    Unknown,
    //树的平衡性未知
    Balanced, //树已确定平衡
}

enum DeleteValue<T: PartialOrd> {
    Min,
    //匹配最小节点
    Max,
    //匹配最大节点
    Val(T),
    //匹配给定值
    Del(AvlTreeNode<T>), //返回被删除节点
}

impl<T: PartialOrd> PartialEq<Box<TreeNode<T>>> for DeleteValue<T> {
    fn eq(&self, other: &Box<TreeNode<T>>) -> bool {
        match self {
            Min => other.left.is_none(),
            Max => other.right.is_none(),
            Val(v) => v == &other.val,
            _ => false,
        }
    }
}

impl<T: PartialOrd> PartialOrd<Box<TreeNode<T>>> for DeleteValue<T> {
    fn partial_cmp(&self, other: &Box<TreeNode<T>>) -> Option<Ordering> {
        match self {
            Min => Some(Ordering::Less),
            Max => Some(Ordering::Greater),
            Val(v) => v.partial_cmp(&other.val),
            _ => None,
        }
    }
}

trait __AvlTree<T: PartialOrd> {
    fn right_rotate(&mut self);
    // 右旋转 - ll
    fn left_rotate(&mut self);
    // 左旋转 - rr
    fn rotate_lr(&mut self);
    fn rotate_rl(&mut self);
    fn update_height(&mut self);
    fn balance_factor(&self) -> i32;
    fn do_insert(&mut self, val: T) -> InnerResult;
    fn do_delete(&mut self, val: &mut DeleteValue<T>) -> InnerResult;
}

pub trait AvlTree<T: PartialOrd> {
    fn new(val: T) -> Self;
    fn height(&self) -> i32;
    fn insert(&mut self, val: T);
    fn delete(&mut self, val: T) -> Self;
}

impl<T: PartialOrd> __AvlTree<T> for AvlTreeNode<T> {
    //         y                            x
    //        / \     Right Rotation       / \
    //       x  T4    ==============>     z   y
    //      / \                          /\ / \
    //     z  T3                        1 2 3  4
    //   T1 T2
    fn right_rotate(&mut self) {
        match self {
            Some(root) => {  // y is root
                // 1. 拿到root的左侧子树，即x分支(此时左侧子树已经剥离)
                let left = &mut root.left.take();
                match left {
                    // 如果左侧子树(x-branch)有东西
                    Some(node) => {
                        // 2. T3连接至y的左侧(root的左侧和x的右侧互换)
                        // root.left=x.right & x.right=root.left
                        swap(&mut root.left, &mut node.right);
                        self.update_height();  // 更新高度
                        // 此时self是y-(T3 & T4)
                        // 3. 将y连接至x分支的右侧(此时root变为了x)
                        swap(self, &mut node.right);
                        // 4. 将重新整合好的x分支(left变量)赋给self
                        // 此时self是整合好之后的x分支
                        swap(self, left);
                        self.update_height();
                    }
                    None => unreachable!(),
                }
            }
            None => unreachable!(),
        }
    }

    //         y                            x
    //        / \     Left  Rotation       / \
    //       T4  x    ==============>     y   z
    //          / \                      / \ / \
    //         T3  z                    4  3 2  1
    //           T2 T1
    fn left_rotate(&mut self) {
        match self {
            Some(root) => { // 此时root是y
                // 1. 拿到y的右侧子树，即x分支(此时子树已经剥离)
                let right = &mut root.right.take();
                match right {
                    // 如果x分支不为空
                    Some(node) => {
                        // 2. 将x的左侧和y的右侧交换(即 y-(T4 & T3))
                        swap(&mut root.right, &mut node.left);
                        self.update_height();
                        // 此时self是y-(T4 & T3)
                        // 3.将x的左侧连接上self(即y分支)，此时root变为x
                        swap(self, &mut node.left);
                        // 4.将重新整合好的x分支赋值给right变量
                        // 此时self是x分支
                        swap(self, right);
                        self.update_height();
                    }
                    None => unreachable!(),
                }
            }
            None => unreachable!(),
        }
    }

    fn rotate_lr(&mut self) {
        match self {
            Some(root) => {
                root.left.left_rotate();
                self.right_rotate();
            }
            None => unreachable!(),
        }
    }

    fn rotate_rl(&mut self) {
        match self {
            Some(root) => {
                root.right.right_rotate();
                self.left_rotate();
            }
            None => unreachable!(),
        }
    }

    fn update_height(&mut self) {
        match self {
            None => {}
            // 找到左侧子树和右侧子树中最高的高度，再加上本身的1，就是自己的高度
            Some(node) => node.height = max(node.left.height(), node.right.height()) + 1,
        }
    }

    fn balance_factor(&self) -> i32 {
        match self {
            None => 0,
            // 平衡因子 = 左侧子树高度 - 右侧子树高度
            Some(node) => node.left.height() - node.right.height(),
        }
    }

    fn do_insert(&mut self, val: T) -> InnerResult {
        match self {
            //如果某个指定位置没有节点，就新建一个放在这里
            None => {
                *self = Self::new(val);
                Unknown
            }
            //递归插入
            Some(root) => {
                //重复数据
                if val == root.val {
                    Balanced
                } else if val < root.val {
                    // 目标值 < 当前节点值，向左侧子树寻找位置
                    match root.left.do_insert(val) {
                        Balanced => Balanced,
                        NotBalanced => {
                            // 当"平衡因子"绝对值大于1的时候就是不平衡，此时是正数，代表左侧不平衡
                            if self.balance_factor() == 2 {
                                match NotBalanced {
                                    Left => self.right_rotate(), // Case: LeftLeft - ll
                                    Right => self.rotate_lr(), // Case LeftRight - lr
                                    _ => unreachable!(), //返回 `Unknown` 的时候当前节点必定是平衡的
                                }
                                Balanced  // 操作之后树已经平衡
                            } else if self.height() == {
                                // 验证此时(after rotate)的高度是否和节点内部记录的一样
                                self.update_height();
                                self.height()
                            } {
                                // 这里相当于 else if self.height = self.height {Balanced}
                                Balanced
                            } else {
                                Left  // 如果插入之后既平衡且高度一致，直接记录"左侧完成插入"
                            }
                        }
                    }
                    //进入右子树递归插入
                } else {
                    match root.right.do_insert(val) {
                        Balanced => Balanced,
                        NotBalanced => {
                            // 当"平衡因子"绝对值大于1的时候就是不平衡，此时是负数，代表右侧不平衡
                            if self.balance_factor() == -2 {
                                match NotBalanced {
                                    Left => self.rotate_rl(),  // case: RightLeft - rl
                                    Right => self.left_rotate(),  // case: RightRight - rr
                                    _ => unreachable!(),
                                }
                                Balanced
                            } else if self.height() == {
                                self.update_height();
                                self.height()
                            } {
                                Balanced
                            } else {
                                Right
                            }
                        }
                    }
                }
            }
        }
    }

    fn do_delete(&mut self, val: &mut DeleteValue<T>) -> InnerResult {
        match self {
            // 如果这个地方没有值，那就"什么都不做"
            None => {
                *val = Del(None); // 用delete(None)代表do nothing
                Balanced
            }
            // 如果有
            Some(root) => {
                // 先搞到以这个节点为跟的树(或子树)的高度，存起来备用
                let height = root.height;
                // case 1:如果要找的就是当前这个
                if val == root {
                    if root.left.is_some() {
                        //左右子树均非空
                        if root.right.is_some() {
                            if root.left.height() > root.right.height() {
                                *val = Max;
                                root.left.do_delete(val);
                                match val {
                                    Del(Some(node)) => {
                                        swap(&mut root.val, &mut node.val);
                                    }
                                    _ => unreachable!(),
                                }
                            } else {
                                *val = Min;
                                root.right.do_delete(val);
                                match val {
                                    Del(Some(x)) => {
                                        swap(&mut root.val, &mut x.val);
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            //左子树非空，右子树为空
                        } else {
                            let mut left = root.left.take();
                            swap(self, &mut left);
                            *val = Del(left);
                        }
                        //左子树为空，右子树非空或为空
                    } else {
                        let mut right = root.right.take();
                        swap(self, &mut right);
                        *val = Del(right);
                    }
                    self.update_height();
                    //进入左子树递归删除
                } else if val < root {
                    match root.left.do_delete(val) {
                        Balanced => return Balanced,
                        Unknown => {
                            if self.balance_factor() == -2 {
                                let right = self.as_ref().unwrap().right.as_ref().unwrap();
                                if right.left.height() > right.right.height() {
                                    self.rotate_rl();
                                } else {
                                    self.left_rotate();
                                }
                            } else {
                                self.update_height();
                            }
                        }
                        _ => unreachable!(),
                    }
                    //进入右子树递归删除
                } else {
                    match root.right.do_delete(val) {
                        Balanced => return Balanced,
                        Unknown => {
                            if self.balance_factor() == 2 {
                                let left = self.as_ref().unwrap().left.as_ref().unwrap();
                                if left.left.height() >= left.right.height() {
                                    self.right_rotate();
                                } else {
                                    self.rotate_lr();
                                }
                            } else {
                                self.update_height();
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                if self.height() == height {
                    Balanced
                } else {
                    Unknown
                }
            }
        }
    }
}

impl<T: PartialOrd> AvlTree<T> for AvlTreeNode<T> {
    fn new(val: T) -> Self {
        Some(Box::new(TreeNode {
            val,
            height: 1,
            left: None,
            right: None,
        }))
    }

    fn height(&self) -> i32 {
        match self {
            None => 0,
            Some(x) => x.height,
        }
    }

    fn insert(&mut self, val: T) {
        self.do_insert(val);
    }

    fn delete(&mut self, val: T) -> Self {
        let mut val = Val(val);
        self.do_delete(&mut val);
        match val {
            Del(x) => x,
            _ => unreachable!(),
        }
    }
}