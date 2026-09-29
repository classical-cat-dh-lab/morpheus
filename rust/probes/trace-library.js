// Read-only Wasm checkpoints. No C/Rust stack allocation or libc logging call.
addToLibrary({
  morph_trace_word: function(tag, pointer) {
    var fields = [UTF8ToString(tag)];
    function text(p,n) { var s='';for(var i=0;i<n&&HEAPU8[p+i];i++)s+=HEAPU8[p+i].toString(16).padStart(2,'0');fields.push(s); }
    function meta(p) {
      var bits=HEAPU32[p>>>2],offset=0;
      [3,4,4,3,3,6,2,4].forEach(function(width){fields.push((bits>>>offset)&((1<<width)-1));offset+=width;});
      fields.push(HEAPU32[(p+4)>>>2],HEAPU32[(p+8)>>>2],HEAP16[(p+12)>>>1],HEAPU32[(p+16)>>>2]);
      for(var i=0;i<12;i++)fields.push(HEAPU8[p+20+i]);text(p+32,21);
    }
    function gs(p){meta(p);text(p+53,60);}
    if(!pointer){fields.push('null');}else{
      var p=pointer;meta(p);var total=HEAP32[(p+60)>>>2];fields.push(HEAP32[(p+56)>>>2],total);
      [64,704,764,824,884].forEach(function(o){text(p+o,60);});
      [124,240,356,472,588].forEach(function(o){gs(p+o);});
      var analyses=HEAPU32[(p+948)>>>2];
      for(var j=0;j<total;j++){var a=analyses+j*1116;fields.push('analysis');meta(a);[53,816,876,936,996,113,173].forEach(function(o){text(a+o,60);});[236,352,468,584,700].forEach(function(o){gs(a+o);});}
    }
    (Module['morphTrace']||(Module['morphTrace']=[])).push(fields.join('|')+'\n');
  }
});
