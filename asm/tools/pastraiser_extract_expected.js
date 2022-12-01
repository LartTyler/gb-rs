(() => {
  // Flag to indicate that the table has already been simplified by pastraiser_simple_cycles.js
  const IS_SIMPLIFIED = true;

  const CYCLE_DIVISOR = IS_SIMPLIFIED ? 1 : 4;
  const TABLES = [
    'table:nth-of-type(1)',
    'table:nth-of-type(2)',
  ];
  
  for (const selector of TABLES)
    console.log(JSON.stringify(processTable(selector)));
  
  function processTable(selector) {
    const nodes = document.querySelector(selector).querySelectorAll('tbody > tr:not(:first-child) > td:not(:first-child)');
    
    let opcode = -1;
    const output = [];
    
    for (const node of nodes) {
      ++opcode;
      
      if (node.childElementCount === 0)
        continue;
      
      const infoNode = node.childNodes[2];
      let [width, cycles] = infoNode.textContent.split(/\s+/);
      
      output[opcode] = {
        opcode,
        label: node.childNodes[0].textContent,
        width: parseInt(width, 10),
        cycles: parseCycles(cycles),
      };
    }
    
    return output;
  }
    
  function parseCycles(input) {
  	if (input.indexOf('/') !== -1) {
      let [min, max] = input.split('/').map(n => parseInt(n.trim(), 10) / CYCLE_DIVISOR);
	
	  if (!IS_SIMPLIFIED) {
		  let tmp = min;
		  min = max;
		  max = tmp;
	  }
      
      return {kind: 'variable', min, max};
    } else {
      return {kind: 'fixed', value: parseInt(input, 10) / CYCLE_DIVISOR};
    }
  }
})();
