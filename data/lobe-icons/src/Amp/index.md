---
nav: Components
group: Application
title: Amp
atomId: Amp
description: https://ampcode.com
---

## Icons

```tsx
import { Amp } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <Amp size={64} />
    <Amp.Color size={64} />
  </Flexbox>
);
```

## Text

```tsx
import { Amp } from '@lobehub/icons';

export default () => <Amp.Text size={48} />;
```

## Combine

```tsx
import { Amp } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} align={'flex-start'}>
    <Amp.Combine size={64} />
    <Amp.Combine size={64} type={'color'} />
  </Flexbox>
);
```

## Avatars

```tsx
import { Amp } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <Amp.Avatar size={64} />
    <Amp.Avatar size={64} shape={'square'} />
  </Flexbox>
);
```

## Colors

```tsx
import { Amp } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

import ColorPreview from '../components/ColorPreview';

export default () => (
  <Flexbox gap={16} horizontal>
    <ColorPreview color={Amp.colorPrimary} />
  </Flexbox>
);
```
