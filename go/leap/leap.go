package leap

const testVersion = 2

// IsLeapYear returns true if year is a leap year
func IsLeapYear(year int) bool {
	if year % 4 == 0 {
		if year % 400 == 0 {
			return true
		} else if year % 100 == 0 {
			return false
		} else {
			return true
		}
	}
	return false
}