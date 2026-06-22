---
nav: Components
group: Application
title: OpenHands
atomId: OpenHands
description: https://openhands.dev
---

## Icons

```tsx
import { OpenHands } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <OpenHands size={64} />
    <OpenHands.Color size={64} />
  </Flexbox>
);
```

## Text

```tsx
import { OpenHands } from '@lobehub/icons';

export default () => <OpenHands.Text size={48} />;
```

## Combine

```tsx
import { OpenHands } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} align={'flex-start'}>
    <OpenHands.Combine size={64} />
    <OpenHands.Combine size={64} type={'color'} />
  </Flexbox>
);
```

## Avatars

```tsx
import { OpenHands } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <OpenHands.Avatar size={64} />
    <OpenHands.Avatar size={64} shape={'square'} />
  </Flexbox>
);
```

## Colors

```tsx
import { OpenHands } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

import ColorPreview from '../components/ColorPreview';

export default () => (
  <Flexbox gap={16} horizontal>
    <ColorPreview color={OpenHands.colorPrimary} />
  </Flexbox>
);
```
