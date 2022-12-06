use ndarray::{Array, Array2, ArrayBase, Axis, Data, Ix2};
use ndarray_rand::RandomExt;
use rand::distributions::{Distribution, Normal};

// Define the structure of the neural network.
const INPUT_SIZE: usize = 28 * 28; // 784
const HIDDEN_SIZE: usize = 100;
const OUTPUT_SIZE: usize = 10;

// Define the weights and biases of the neural network.
let mut w1 = Array::random((INPUT_SIZE, HIDDEN_SIZE), Normal::new(0., 1.));
let mut b1 = Array::zeros(HIDDEN_SIZE);
let mut w2 = Array::random((HIDDEN_SIZE, OUTPUT_SIZE), Normal::new(0., 1.));
let mut b2 = Array::zeros(OUTPUT_SIZE);

// Define the activation function of the neural network.
fn sigmoid(x: f64) -> f64 {
    1. / (1. + (-x).exp())
}

// Define the forward pass of the neural network.
fn forward(x: &ArrayBase<Ix2, f64>) -> Array2<f64> {
    let z1 = x.dot(&w1) + &b1;
    let a1 = z1.mapv(sigmoid);
    let z2 = a1.dot(&w2) + &b2;
    let a2 = z2.mapv(sigmoid);
    a2
}

// Define the loss function of the neural network.
fn loss(pred: &ArrayBase<Ix2, f64>, label: &ArrayBase<Ix2, f64>) -> f64 {
    let diff = label - pred;
    diff.mapv(|x| x * x).sum()
}

// Define the backward pass of the neural network.
fn backward(x: &ArrayBase<Ix2, f64>, label: &ArrayBase<Ix2, f64>) {
    // Compute the gradient of the loss function with respect to the output of the network.
    let pred = forward(x);
    let dpred = 2. * (pred - label);

    // Compute the gradient of the loss function with respect to the weights and biases of the network.
    let dw2 = a1.t().dot(&dpred);
    let db2 = dpred.sum
