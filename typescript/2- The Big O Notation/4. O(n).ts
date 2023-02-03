function linearSearch(arr: number[], target: number) {
  // O(n) number of operations grows linearly with the size of the input
  for (let i = 0; i < arr.length; i++) {
    if (arr[i] === target) {
      return i;
    }
  }

  return -1;
}
