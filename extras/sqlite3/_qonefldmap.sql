.separator ","
.headers on
SELECT a.mapid, a.chgnr, b.ctmrl, b.messg, b.mvers, b.idocm, a.sgmid, a.targt, a.sourc, a.rcond, a.commt, a.sampl
FROM fields AS a LEFT JOIN indix AS b ON a.mapid = b.mapid and a.chgnr = b.chgnr
where a.targt like '%KTEXT%' and b.messg in ('delfor','deljit', '830', '850', '860', '862') order by a.sgmid, a.grpid, a.targt;