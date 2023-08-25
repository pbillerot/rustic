/**
 * Graphique du jecompte
 * https://www.chartjs.org/docs/latest/
 * red green purple orange teal pink yellow blue brown olive violet
 */

// 7 Couleurs de fond des barres
const backgroundColors = [
  'rgba(255, 99, 132, 0.2)',  // red
  'rgba(54, 162, 235, 0.2)',  // blue
  'rgba(0, 255, 255, 0.2)',  // cyan
  'rgba(75, 192, 192, 0.2)',  // green
  'rgba(153, 102, 255, 0.2)', // Purple
  'rgba(255, 159, 64, 0.2)',  // orange
  'rgba(128, 128, 128, 0.2)' // gris
];
// 7 couleurs de bordure des barres
const borderColors = [
  'rgba(255, 99, 132)',  // red
  'rgba(54, 162, 235)',  // blue
  'rgba(0, 255, 255)',  // cyan
  'rgba(75, 192, 192)',  // green
  'rgba(153, 102, 255)', // Purple
  'rgba(255, 159, 64)',  // orange
  'rgba(128, 128, 128)' // gris
];

// Boucle de traitement pour dessiner les graphiques de la page
var elements = document.querySelectorAll('.chart-jecompte');
elements.forEach(function (element) {
  drawChart(element);
});

function drawChart(element) {
  // Données collectées
  const labels = element.dataset.labels.split(',');
  const cumuls = element.dataset.cumuls.split(',');
  const colors = element.dataset.couleurs.split(',');
  // Tri des couleurs dans bct fct
  var bct = Array(colors.length);
  var fct = Array(colors.length);
  for (i=0; i < colors.length; i++ ) {
    bct[i] = backgroundColors[colors[i]];
    fct[i] = borderColors[colors[i]];
  }

  // Construction du graphique
  const myChart = new Chart(element.children.item(0).getContext('2d'), {
    type: 'bar',
    data: {
      labels: labels,
      datasets: [
        {
          type: 'bar',
          axis: 'y',
          backgroundColor: bct,
          borderColor: fct,
          borderWidth: 1,
          label: 'Points',
          data: cumuls
        }
      ]
    },
    options: {
      indexAxis: 'y',
    }
  });
}
