package utils

func MinMax(array []int) (int, int) {
	var max int = array[0]
	var min int = array[0]
	for _, value := range array {
		if max < value {
			max = value
		}
		if min > value {
			min = value
		}
	}
	return min, max
}

func Sum(array []int) int {
	result := 0
	for _, v := range array {
		result += v
	}
	return result
}
