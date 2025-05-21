const Helpers = require('../../../helpers');

  return Math.floor((max + min) / 2);
}
  while (min <= max) {
    var middle = midpoint(min, max);

    else if (array[middle] < target) min = middle + 1;
  }

var sorted = Helpers.sorted;
console.log(binarySearch(sorted, 72, 0, sorted.length - 1));
