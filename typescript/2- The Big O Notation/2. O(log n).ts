function binarySearch(arr: number[], target: number) {
    let left = 0;
    let right = arr.length - 1;
  
    // O(log n) The number of operations grows logarithmically with the size of the input
    while (left <= right) {
      const mid = Math.floor((left + right) / 2);
  
      if (arr[mid] === target) {
        return mid;
      } else if (arr[mid] < target) {
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
  
    return -1;
  }
  