/*
Instead of having `Some` and `if let` to work with the single concept of the
root not having a parent just use 0. That does mean we aren't going to have the
"set_Alpha" here like we do in python but that's really just labelling and prep
work outside the scope of the algorithms.
 */

pub(crate) const SET_7README_PARENTS: [usize; 7] = [0, 1, 1, 1, 2, 2, 3];
pub(crate) const SET_7README_CHILDREN: [usize; 7] = [1, 2, 3, 4, 5, 6, 7];

pub(crate) const SET_RUSKEY_PARENTS: [usize; 8] = [0, 1, 2, 1, 4, 4, 6, 6];
pub(crate) const SET_RUSKEY_CHILDREN: [usize; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

pub(crate) const SET_13M_PARENTS: [usize; 13] = [0, 1, 2, 3, 4, 5, 5, 1, 8, 9, 8, 11, 11];
pub(crate) const SET_13M_CHILDREN: [usize; 13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

pub(crate) const SET_3B_PARENTS: [usize; 3] = [0, 1, 2];
pub(crate) const SET_3B_CHILDREN: [usize; 3] = [1, 2, 3];

pub(crate) const SET_3W_PARENTS: [usize; 3] = [0, 1, 1];
pub(crate) const SET_3W_CHILDREN: [usize; 3] = [1, 2, 3];

pub(crate) const SET_3D_PARENTS: [usize; 3] = [0, 1, 2];
pub(crate) const SET_3D_CHILDREN: [usize; 3] = [1, 2, 3];

pub(crate) const SET_7B_PARENTS: [usize; 7] = [0, 1, 2, 2, 1, 5, 5];
pub(crate) const SET_7B_CHILDREN: [usize; 7] = [1, 2, 3, 4, 5, 6, 7];

pub(crate) const SET_7W_PARENTS: [usize; 7] = [0, 1, 1, 1, 1, 1, 1];
pub(crate) const SET_7W_CHILDREN: [usize; 7] = [1, 2, 3, 4, 5, 6, 7];

pub(crate) const SET_7D_PARENTS: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
pub(crate) const SET_7D_CHILDREN: [usize; 7] = [1, 2, 3, 4, 5, 6, 7];

pub(crate) const SET_15B_PARENTS: [usize; 15] = [0, 1, 2, 3, 3, 2, 6, 6, 1, 9, 10, 10, 9, 13, 13];
pub(crate) const SET_15B_CHILDREN: [usize; 15] =
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

pub(crate) const SET_15W_PARENTS: [usize; 15] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
pub(crate) const SET_15W_CHILDREN: [usize; 15] =
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

pub(crate) const SET_15D_PARENTS: [usize; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
pub(crate) const SET_15D_CHILDREN: [usize; 15] =
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

pub(crate) const SET_31B_PARENTS: [usize; 31] = [
    0, 1, 2, 3, 4, 4, 3, 7, 7, 2, 10, 11, 11, 10, 14, 14, 1, 17, 18, 19, 19, 18, 22, 22, 17, 25,
    26, 26, 25, 29, 29,
];
pub(crate) const SET_31B_CHILDREN: [usize; 31] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31,
];

pub(crate) const SET_31W_PARENTS: [usize; 31] = [
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];
pub(crate) const SET_31W_CHILDREN: [usize; 31] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31,
];

pub(crate) const SET_53X_PARENTS: [usize; 53] = [
    0, 1, 2, 3, 4, 5, 5, 4, 8, 8, 3, 11, 12, 12, 11, 15, 15, 2, 18, 19, 20, 20, 19, 23, 23, 18, 26,
    27, 27, 26, 30, 30, 1, 33, 34, 35, 36, 36, 35, 39, 39, 34, 42, 43, 43, 42, 46, 46, 33, 49, 50,
    51, 51,
];
pub(crate) const SET_53X_CHILDREN: [usize; 53] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53,
];

pub(crate) const SET_63B_PARENTS: [usize; 63] = [
    0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14,
    14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25, 25, 26,
    26, 27, 27, 28, 28, 29, 29, 30, 30, 31, 31,
];
pub(crate) const SET_63B_CHILDREN: [usize; 63] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
];

pub(crate) fn get_sample_data(sample_size: &str) -> (usize, Vec<usize>, Vec<usize>) {
    let root = 1;
    let mut parents = vec![];
    let mut children = vec![];

    if sample_size == "set_7Readme" {
        parents = SET_7README_PARENTS.to_vec();
        children = SET_7README_CHILDREN.to_vec();
    } else if sample_size == "set_Ruskey" {
        parents = SET_RUSKEY_PARENTS.to_vec();
        children = SET_RUSKEY_CHILDREN.to_vec();
    } else if sample_size == "set_13M" {
        parents = SET_13M_PARENTS.to_vec();
        children = SET_13M_CHILDREN.to_vec();
    } else if sample_size == "set_3B" {
        parents = SET_3B_PARENTS.to_vec();
        children = SET_3B_CHILDREN.to_vec();
    } else if sample_size == "set_3D" {
        parents = SET_3D_PARENTS.to_vec();
        children = SET_3D_CHILDREN.to_vec();
    } else if sample_size == "set_3W" {
        parents = SET_3W_PARENTS.to_vec();
        children = SET_3W_CHILDREN.to_vec();
    } else if sample_size == "set_7B" {
        parents = SET_7B_PARENTS.to_vec();
        children = SET_7B_CHILDREN.to_vec();
    } else if sample_size == "set_7D" {
        parents = SET_7D_PARENTS.to_vec();
        children = SET_7D_CHILDREN.to_vec();
    } else if sample_size == "set_7W" {
        parents = SET_7W_PARENTS.to_vec();
        children = SET_7W_CHILDREN.to_vec();
    } else if sample_size == "set_15B" {
        parents = SET_15B_PARENTS.to_vec();
        children = SET_15B_CHILDREN.to_vec();
    } else if sample_size == "set_15D" {
        parents = SET_15D_PARENTS.to_vec();
        children = SET_15D_CHILDREN.to_vec();
    } else if sample_size == "set_15W" {
        parents = SET_15W_PARENTS.to_vec();
        children = SET_15W_CHILDREN.to_vec();
    } else if sample_size == "set_31B" {
        parents = SET_31B_PARENTS.to_vec();
        children = SET_31B_CHILDREN.to_vec();
    } else if sample_size == "set_31W" {
        parents = SET_31W_PARENTS.to_vec();
        children = SET_31W_CHILDREN.to_vec();
    } else if sample_size == "set_53X" {
        parents = SET_53X_PARENTS.to_vec();
        children = SET_53X_CHILDREN.to_vec();
    } else if sample_size == "set_63B" {
        parents = SET_63B_PARENTS.to_vec();
        children = SET_63B_CHILDREN.to_vec();
    }

    if parents.is_empty() || children.is_empty() {
        panic!("Sample data {sample_size:?} not found.");
    }

    (root, parents, children)
}
