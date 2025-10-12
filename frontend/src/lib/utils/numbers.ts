export const formatNumber = (number: number, decimals = false) => {
  const whole = Math.floor(number)
  // format whole with thousands separator ","
  let value = whole.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',')

  if (decimals) {
    // format fraction with two decimal places
    const fraction = (number - whole).toFixed(2).substring(2)
    value = `${value}.${fraction}`
  }

  return value
}
