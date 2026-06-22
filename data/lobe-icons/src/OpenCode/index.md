---
nav: Components
group: Application
title: opencode
atomId: OpenCode
description: https://opencode.ai
---

## Icons

```tsx
import { OpenCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => <OpenCode size={64} />;
```

## Text

```tsx
import { OpenCode } from '@lobehub/icons';

export default () => <OpenCode.Text size={48} />;
```

## Combine

```tsx
import { OpenCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} align={'flex-start'}>
    <OpenCode.Combine size={64} />
  </Flexbox>
);
```

## Avatars

```tsx
import { OpenCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <OpenCode.Avatar size={64} />
    <OpenCode.Avatar size={64} shape={'square'} />
  </Flexbox>
);
```

## Colors

```tsx
import { OpenCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

import ColorPreview from '../components/ColorPreview';

export default () => (
  <Flexbox gap={16} horizontal>
    <ColorPreview color={OpenCode.colorPrimary} />
  </Flexbox>
);
```
