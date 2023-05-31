-- The view for the adsb_messages kafka topic

create materialized view adsb_messages_view to adsb_messages as
select * replace (
    if(isNull(emergency), 0, emergency) as emergency,
    if(isNull(airground), 0, airground) as airground,
    if(isNull(nav_altitude_src), 0, nav_altitude_src) as nav_altitude_src,
    if(isNull(sil_type), 0, sil_type) as sil_type,
    if(isNull(pos_type), 0, pos_type) as pos_type
    )
from adsb_messages_queue
;