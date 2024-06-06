.separator ","
.headers on
SELECT a.mapid, a.chgnr, b.ctmrl, b.messg, b.mvers, b.idocm, a.grpid, a.sgmid, a.targt, a.sourc, a.rcond, a.commt, a.sampl
FROM fields AS a LEFT JOIN indix AS b ON a.mapid = b.mapid and a.chgnr = b.chgnr
where b.messg in ('812', '820', '824', '846', 'aperak', 'contrl', 'invrpt', 'recadv') 
order by a.mapid, a.chgnr, a.rowno;