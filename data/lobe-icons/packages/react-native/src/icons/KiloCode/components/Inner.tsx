import React, { memo } from 'react';
import { Path, Svg } from 'react-native-svg';

import type { RNIconProps } from '@/features';

const Icon = memo<RNIconProps>(({ size = 24, style, color = '#000000', ...rest }) => {
  return (
    <Svg
      color={color}
      fillRule="evenodd"
      height={size}
      style={style}
      viewBox="0 0 24 24"
      width={size}
      {...rest}
    >
      <Path
        d="M16.4 21.016h3.667V24h-4.61l-1.99-2.025v-4.689H16.4v3.73zm6.6 0h-2.933v-3.73H16.4v-2.984h4.61L23 16.327v4.689zm-12.467-4.35H7.6v-2.984h2.933v2.984zM1 13.682h2.933v6.714h6.6v2.984H2.99L1 21.355v-7.673zm22-5.968v2.984h-9.533V7.714h3.278v-3.73h-3.278V1h4.22l1.991 2.025v4.689H23zM3.933 4.357H7.6l2.933 2.984v3.357H7.6V7.341H3.933v3.357H1V1h2.933v3.357zm6.6 0H7.6V1h2.933v3.357z"
        fill={color}
      />
    </Svg>
  );
});

Icon.displayName = 'KiloCodeInner';

export default Icon;
