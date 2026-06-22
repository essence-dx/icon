---
nav: Components
group: Application
title: Kilo Code
atomId: KiloCode
description: https://kilo.ai
---

## Icons

```tsx
import { KiloCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => <KiloCode size={64} />;
```

## Text

```tsx
import { KiloCode } from '@lobehub/icons';

export default () => <KiloCode.Text size={48} />;
```

## Combine

```tsx
import { KiloCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} align={'flex-start'}>
    <KiloCode.Combine size={64} />
  </Flexbox>
);
```

## Avatars

```tsx
import { KiloCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <KiloCode.Avatar size={64} />
    <KiloCode.Avatar size={64} shape={'square'} />
  </Flexbox>
);
```

## Colors

```tsx
import { KiloCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

import ColorPreview from '../components/ColorPreview';

export default () => (
  <Flexbox gap={16} horizontal>
    <ColorPreview color={KiloCode.colorPrimary} />
  </Flexbox>
);
```
