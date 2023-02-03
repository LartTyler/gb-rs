(() => {
  const TABLES = [
    'table:nth-of-type(1)',
    'table:nth-of-type(2)',
  ];
  
  for (const selector of TABLES)
    processTable(selector);
  
  function processTable(selector) {
    const nodes = document.querySelector(selector).querySelectorAll('tbody > tr:not(:first-child) > td:not(:first-child)');
    
    for (const node of nodes) {
      if (node.childElementCount === 0)
        continue;
      
      const infoNode = node.childNodes[2];
      let [width, cycles] = infoNode.textContent.split(/\s+/);
      
      if (cycles.indexOf('/') !== -1) {
        const [max, min] = cycles.split('/').map(n => parseInt(n.trim(), 10) / 4);
        cycles = `${min}/${max}`;
      } else {
        cycles = (parseInt(cycles, 10) / 4).toString();
      }
      
      infoNode.textContent = `${width}  ${cycles}`;
    }
  }
})();
