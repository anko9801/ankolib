pub trait Graph {
    fn len(&self) -> usize;
    fn adj(&self, v: usize, f: impl FnMut(usize));
}

impl Graph for Vec<Vec<usize>> {
    fn len(&self) -> usize {
        self.len()
    }
    fn adj(&self, v: usize, f: impl FnMut(usize)) {
        self[v].iter().copied().for_each(f);
    }
}

pub trait WGraph: Graph {
    type W;
    fn adj_w(&self, v: usize, f: impl FnMut(usize, Self::W));
}

impl WGraph for Vec<Vec<usize>> {
    type W = usize;
    fn adj_w(&self, v: usize, mut f: impl FnMut(usize, Self::W)) {
        self[v].iter().for_each(|&v| f(v, 1));
    }
}

impl<W> Graph for Vec<Vec<(usize, W)>> {
    fn len(&self) -> usize {
        self.len()
    }
    fn adj(&self, v: usize, mut f: impl FnMut(usize)) {
        self[v].iter().for_each(|&(v, _)| f(v))
    }
}

impl<W: Copy> WGraph for Vec<Vec<(usize, W)>> {
    type W = W;
    fn adj_w(&self, v: usize, mut f: impl FnMut(usize, W)) {
        self[v].iter().for_each(|&(v, w)| f(v, w));
    }
}
